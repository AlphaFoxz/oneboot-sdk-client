pipeline {
  agent any
  stages {
    stage('verify env') {
      steps {
        sh 'cargo --version'
        sh 'pnpm -v'
      }
    }
    stage('install dependencies') {
      steps {
        sh 'pnpm install'
        sh 'cargo check --manifest-path ./src-tauri/Cargo.toml'
      }
    }
    stage('compile') {
      steps {
        sh 'pnpm tauri build'
      }
    }
  }
}