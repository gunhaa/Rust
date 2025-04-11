![ferris](/docs/images/rustacean-flat-happy.png)

- 설치 확인
```shell
$ rustc --version
rustc 1.84.0 (9fc6b4312 2025-01-07)
```
- 공식 문서 확인
```shell
$ rustup doc
```
- 컴파일 & 실행
```shell
$ rustc main.rs
> .\main.exe
```
- Rust 스타일 포매팅 사용
```
$ rustfmt main.rs
```
- cargo 버전 확인(러스트 빌드 시스템 및 패키지 매니저)
```shell
$ cargo --version
cargo 1.84.0 (66221abde 2024-11-19)
```
- cargo 프로젝트 생성(git저장소가 없다면 초기화까지 시켜준다)
```shell
$ cargo new hello_cargo
```
- cargo를 이용한 빌드/실행
```shell
$ cargo build
> .\target\debug\hello_cargo.exe
```
- cargo 빌드, 실행 명령어
```shell
$ cargo run
```
- 컴파일 가능한지만 확인
```shell
$ cargo check
```
- 릴리스 빌드 생성
```shell
$ cargo build --release
```