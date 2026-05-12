use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("sprout").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sprout is a Rust-based CLI"));
}

#[test]
fn test_cli_generate_resource_fails_outside_project() {
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("sprout").unwrap();

    cmd.current_dir(temp.path())
        .arg("g")
        .arg("resource")
        .arg("user")
        .env("SPROUT_NON_INTERACTIVE", "1")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "must be run from a Spring Boot project root",
        ));
}

#[test]
fn test_cli_generate_resource_non_interactive() {
    let temp = TempDir::new().unwrap();

    let main_java = temp.path().join("src/main/java/com/acme/demo");
    fs::create_dir_all(&main_java).unwrap();
    fs::write(
        main_java.join("DemoApplication.java"),
        "package com.acme.demo;\nimport org.springframework.boot.autoconfigure.SpringBootApplication;\n@SpringBootApplication\npublic class DemoApplication {}"
    ).unwrap();

    fs::write(temp.path().join("build.gradle"), "dependencies {}").unwrap();

    let mut cmd = Command::cargo_bin("sprout").unwrap();

    cmd.current_dir(temp.path())
        .arg("g")
        .arg("resource")
        .arg("product")
        .env("SPROUT_NON_INTERACTIVE", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("Done"));

    assert!(main_java.join("product/entity/Product.java").exists());
    assert!(main_java
        .join("product/controller/ProductController.java")
        .exists());

    let test_java = temp.path().join("src/test/java/com/acme/demo");
    assert!(test_java
        .join("product/service/ProductServiceTest.java")
        .exists());
    assert!(test_java
        .join("product/controller/ProductControllerTest.java")
        .exists());
}

#[test]
fn test_cli_generate_mongo_and_class_style_non_interactive() {
    let temp = TempDir::new().unwrap();

    let main_java = temp.path().join("src/main/java/com/acme/demo");
    fs::create_dir_all(&main_java).unwrap();
    fs::write(
        main_java.join("DemoApplication.java"),
        "package com.acme.demo;\nimport org.springframework.boot.autoconfigure.SpringBootApplication;\n@SpringBootApplication\npublic class DemoApplication {}"
    ).unwrap();

    fs::write(temp.path().join("build.gradle"), "dependencies {}").unwrap();

    let mut cmd = Command::cargo_bin("sprout").unwrap();

    cmd.current_dir(temp.path())
        .arg("g")
        .arg("resource")
        .arg("customer")
        .env("SPROUT_NON_INTERACTIVE", "1")
        .env("SPROUT_PERSISTENCE", "mongo")
        .env("SPROUT_DTO_STYLE", "class")
        .assert()
        .success()
        .stdout(predicate::str::contains("Done"));

    assert!(main_java.join("customer/entity/Customer.java").exists());
    assert!(main_java
        .join("customer/repository/CustomerRepository.java")
        .exists());

    let test_java = temp.path().join("src/test/java/com/acme/demo");
    assert!(test_java
        .join("customer/service/CustomerServiceTest.java")
        .exists());
}
