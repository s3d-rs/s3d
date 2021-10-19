extra["displayName"] = "S3D CODEGEN"

val smithyVersion: String by project

plugins {
    id("software.amazon.smithy").version("0.5.3")
}

dependencies {
    implementation(project(":codegen"))
    implementation(project(":codegen-server"))
    // implementation(project(":aws:sdk-codegen"))
    implementation("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
}

// tasks["assemble"].dependsOn("smithyBuildJar")
