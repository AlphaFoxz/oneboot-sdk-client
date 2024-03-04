pipeline {
  agent any
  stages {
    stage('verify env') {
      steps {
        script {
          if (sh(script: 'command -v rustc', returnStatus: true) != 0) {
            echo 'Installing Rust...'
            sh "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
          }
        }
        sh '$HOME/.cargo/env'
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