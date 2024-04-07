buildscript {
    val kotlinVersion by extra { "1.6.10" }

    repositories {
        google()
        mavenCentral()
        maven("https://plugins.gradle.org/m2/")
        mavenLocal()
    }

    dependencies {
        classpath("com.android.tools.build:gradle:7.2.2")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:$kotlinVersion")
        classpath("io.github.MatrixDev.android-rust:plugin:0.3.21")
    }
}

allprojects {
    repositories {
        google()
        mavenCentral()
        maven("https://plugins.gradle.org/m2/")
        mavenLocal()
    }
}

tasks.register("clean", Delete::class) {
    delete(rootProject.buildDir)
    rootProject.childProjects.forEach { project ->
        delete(project.value.buildDir)
    }
}
