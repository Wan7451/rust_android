### 集成JNI

在Cargo.toml中添加jni依赖
```toml
[dependencies]
jni = "0.21.1"#jni 依赖
[lib]
crate-type = ["cdylib"]  #导出为动态库
```

##### 动态库和静态库

动态库和静态库是计算机编程中常见的两种库形式，它们的主要区别在于链接阶段和运行时的行为。

- 静态库（Static Library）
1. 链接方式：  在编译过程中，静态库的内容会被直接复制并整合到最终生成的目标文件（可执行文件或另一个静态库）中。这意味着静态库中的所有代码和数据都会成为目标文件的一部分。
2. 文件扩展名：在不同平台上静态库有不同的扩展名，如在 Unix/Linux 系统上是 .a，在 Windows 上通常是 .lib。
3. 大小与性能：由于静态库的内容被完整地包含在每个链接它的应用程序中，因此会导致生成的可执行文件较大，但启动速度较快，因为不需要在运行时加载外部库。
4. 更新与部署：如果静态库有更新，那么依赖于它的所有程序都需要重新编译并链接新的版本才能获取更新内容。

- 动态库（Dynamic Library / Shared Library）
1. 链接方式：  动态库在编译时仅记录所依赖的函数符号，在运行时才将库的实际内容加载到内存中。这使得多个程序可以共享同一份库资源，从而节省内存空间。
2. 文件扩展名：动态库在Unix/Linux系统上通常是.so（Shared Object），在Windows系统上是.dll（Dynamic Link Library）。
3. 大小与性能：使用动态库的应用程序体积较小，但在运行时必须能找到相应的库文件并成功加载。动态库可以带来更快的启动速度（如果库已经被其他进程加载），但如果频繁加载和卸载库可能会增加开销。
4. 更新与部署：更新动态库时，只需替换系统或应用目录下的库文件，依赖于该库的所有程序无需重新编译就可以享受到新版本的功能，但也可能导致兼容性问题，如果接口发生了变化。
#依赖管理： 动态库在运行时需要确保所需的库存在于系统的正确路径中，否则会导致“找不到库”的错误。


### JNI方法

#### 原生调用rust 

```rust
#[no_mangle]
extern "C" fn Java_com_wan7451_native_FFICenter_callRustCode(mut env: JNIEnv, class: JClass, input: JString) -> jstring {
}
```

##### 方法解释

1.  #[no_mangle] 一个属性,指示 Rust 编译器不要对特定函数进行名称混淆
2. extern 导出 Rust 函数供其他语言调 
- extern "C" 指定 Rust 函数遵循 C 语言 ABI，这样其他遵循 C ABI 的语言就可以安全地调用这个函数。 
- extern 关键字在 Rust 中起到了桥梁的作用，允许 Rust 代码和其他编程语言之间进行互操作。

3. jni 支持类型
- | Java类型 | JNI类型 | 原始Rust类型 | Rust包装类型（通过jni库） |
   | --- | --- | --- | --- |
   | boolean | jboolean | bool | ——（直接使用） |
   | byte | jbyte | i8 | ——（直接使用） |
   | char | jchar | u16 | ——（直接使用） |
   | short | jshort | i16 | ——（直接使用） |
   | int | jint | i32 | ——（直接使用） |
   | long | jlong | i64 / isize | ——（直接使用） |
   | float | jfloat | f32 | ——（直接使用） |
   | double | jdouble | f64 | ——（直接使用） |
   | String | jstring | jni::sys::jstring | jni::objects::JString |
   | Class | jclass | jni::sys::jclass | jni::objects::JClass |
   | Array (如 int[]) | jintArray | jni::sys::jintArray | jni::array::JArray<i32> |
   | Object | jobject | jni::sys::jobject | jni::objects::JObject |
   | Interface | jobject | jni::sys::jobject | jni::objects::JObject（泛化处理） |
   | Exception | jthrowable | jni::sys::jthrowable | jni::objects::JThrowable |

4. jstring 与 JString 有什么不同
- JString 和 jstring 在JNI编程上下文中指的是相同的概念，但在不同的上下文中可能有不同的含义：
- JNI原生类型（jstring）： 在JNI（Java Native Interface）规范中，jstring 是一种原生类型，它用来表示Java虚拟机内部的一个字符串引用。在C/C++的JNI接口中，jstring 是一个指向Java字符串对象的指针，但它并不是C/C++标准库中的字符串类型，而是JNI接口定义的类型。
- JNI包装类型（JString）： 在一些JNI的第三方库如Rust的jni crate中，为了提供更安全、更易于使用的API，会将JNI的原生类型包装成高级别的类型。在这种情况下，JString 是对 jstring 的封装，提供了更多的方法和特性，使得开发者在Rust中操作Java字符串时更加便捷。比如，jni::objects::JString 类型就是对JNI中的 jstring 类型进行了一层封装。

总结来说，jstring 是JNI规范中定义的原生类型，而 JString 可能是某些高级语言（如Rust）为了更好地与JNI集成而提供的封装类型。在JNI接口的实际使用中，两者通常是相互配合使用的，JString 会通过其方法间接操作底层的 jstring 数据。

#### rust 调用原生

```rust
let result = env.new_string("hello,aaaa").unwrap();
env.call_static_method("com/wan7451/native/FFICenter", "logTest", "(Ljava/lang/String;)V", &[JValueGen::Object(&result)]).unwrap();
```

##### JNI方法描述符
JNI（Java Native Interface）中的方法描述符用于描述Java方法的参数类型和返回类型，遵循以下规则：
1. 类型字符：
- Z：布尔类型（boolean）
- B：字节类型（byte）
- C：字符类型（char）
- S：短整型（short）
- I：整型（int）
- J：长整型（long）
- F：浮点型（float）
- D：双精度浮点型（double）
- [：表示数组，例如 [I 表示整型数组
- L：开始一个类或接口的类型描述，后面跟着完全限定的类名，类名间用 / 分隔，结尾用 ; 结束。例如 Ljava/lang/String; 表示 java.lang.String 类型。
2. 描述符结构：
- 描述符以左括号 ( 开始，表示参数列表开始。
- 参数类型按照参数顺序依次列出。
- 参数列表以右括号 ) 结束。
- 参数列表结束后跟上的类型表示方法的返回类型。
- 特殊类型：
-V：表示 void 类型，即方法没有返回值。
3. 方法描述符示例：
- (I)V：表示一个接受一个整型参数，返回值为 void 的方法。
- (Ljava/lang/String;[I)Z：表示一个接受一个 java.lang.String 类型参数和一个整型数组参数，返回值为 boolean 的方法。
- ()Ljava/lang/String;：表示一个无参数，返回值为 java.lang.String 类型的方法。
- **(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String; 表示方法 public String method(String a, String b) {}** 
4. 对象和数组：
- 对象类型的描述符中，L 后跟随的是类的全限定名，例如 Ljava/lang/Object;。
- 数组类型的描述符中，方括号内包含数组元素的类型，例如 [Ljava/lang/String; 表示 String[] 类型的数组。

总之，JNI方法描述符是一种紧凑的文本格式，用于精确表示Java方法的完整类型信息，以便于JNI方法与Java方法之间进行正确的互操作。

https://github.com/seanmonstar/reqwest/issues/495


https://github.com/MatrixDev/GradleAndroidRustPlugin


OnceCell 在 Rust 中的作用是为了保证某个资源只会被初始化一次，并且在初始化完成后可以安全地在多个线程之间共享。