plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("io.github.MatrixDev.android-rust")
}

android {
    namespace = "com.wan7451.anative"
    compileSdk = 33
    ndkVersion = "25.2.9519653"

    defaultConfig {
        minSdk = 24

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.9.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
}

androidRust {
    module("library") {
        path = file("../rust_library")

        buildType("release") {
            runTests = true
        }
    }
    minimumSupportedRustVersion = "1.62.1"
}