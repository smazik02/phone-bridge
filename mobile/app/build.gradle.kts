plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.compose)
}

android {
    namespace = "com.smazik.phonebridge.mobile"
    compileSdk {
        version = release(36) {
            minorApiLevel = 1
        }
    }

    defaultConfig {
        applicationId = "com.smazik.phonebridge.mobile"
        minSdk = 29
        targetSdk = 36
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            optimization {
                enable = false
            }
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
    buildFeatures {
        compose = true
    }

    sourceSets {
        getByName("main") {
            java.directories.add("../../core/dist/android")
            kotlin.directories.add("../../core/dist/android")
            jniLibs.directories.add("../../core/dist/android/jniLibs")
        }
    }
}

dependencies {
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.compose.material3)
    implementation(libs.androidx.compose.ui)
    implementation(libs.androidx.compose.ui.graphics)
    implementation(libs.androidx.compose.ui.tooling.preview)
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)

    testImplementation(libs.junit)

    androidTestImplementation(platform(libs.androidx.compose.bom))
    androidTestImplementation(libs.androidx.compose.ui.test.junit4)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(libs.androidx.junit)

    debugImplementation(libs.androidx.compose.ui.test.manifest)
    debugImplementation(libs.androidx.compose.ui.tooling)
}

mapOf(
    "Debug" to "",
    "Release" to "--release"
).forEach { (variant, flag) ->
    tasks.register<Exec>("buildRustCore$variant") {
        description = "Building shared Rust Core with FFI bridge: $variant"

        workingDir = file("../../core")
        commandLine("sh", "-c", "boltffi pack android $flag".trim())

        inputs.dir("../../core/src")
        inputs.file("../../core/Cargo.toml")
        inputs.file("../../core/Cargo.lock")
        inputs.file("../../core/boltffi.toml")

        outputs.dir("../../core/dist/android")
    }

    tasks.matching {
        it.name.startsWith("compile$variant") ||
                it.name.startsWith("merge${variant}Jni") ||
                it.name.startsWith("merge${variant}Native") ||
                it.name == "pre${variant}Build"
    }
        .configureEach {
            dependsOn("buildRustCore$variant")
        }
}
