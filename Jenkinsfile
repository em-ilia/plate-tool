pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    trunk build --release --public-url "cool-stuff/plate-tool-beta/"
                '''
            }
	    }
	stage('Archive') {
		steps {
			zip zipFile: "dist.zip", archive: true, dir: "dist/"
			archiveArtifacts artifacts: "dist.zip", fingerprint: true
		}
	}
	stage('Transfer') {
		steps {
			sh 'echo "put -r dist/" | sftp oracle'
		}
	}
	stage('Deploy') {
		steps {
			sh '''
			ssh oracle "sudo rm -rf /usr/share/nginx/html/plate-tool-beta/ && sudo mv dist /usr/share/nginx/html/plate-tool-beta"
			'''
		}
	}
    }
    post {
	always {
		cleanWs(notFailBuild: true,
			deleteDirs: true,
			cleanWhenNotBuilt: false,
			patterns: [[pattern: 'target/', type: 'EXCLUDE']])
	}
    }
}
