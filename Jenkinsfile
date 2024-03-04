pipeline {
  agent any
  environment {
    CARGO_BIN = '$HOME/.cargo/bin'
  }
  tools {
    nodejs 'Node18'
  }
  stages {
    stage('verify env') {
      steps {
        script {
          if (sh(script: "command -v ${env.CARGO_BIN}/cargo", returnStatus: true) != 0) {
            echo 'Installing Rust...'
            sh "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
          }
        }
        sh "export PATH=\"${env.CARGO_BIN}:$PATH\""
        sh 'cargo --version'
        sh 'npm install -g pnpm'
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