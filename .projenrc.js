const { RustProject } = require('@gplassard/projen-extensions');
const package = require('./package.json');

const project = new RustProject({
    name: 'aws-credentials-switcher',
    cargo: {
        package: {
            authors: ["Gabriel Plassard <gabriel.plassard@gmail.com>"],
            version: package.version,
            edition: "2021",
        },
        dependencies: {
            'dirs': "4.0.0",
            'rust-ini': "0.18.0",
            'exitcode': "1.1.2",
            'structopt': "0.3.26",
            'log': "0.4.16",
            'env_logger': "0.10.0",
        }
    }
});
project.synth();
