//! Monster Group Solana Validator CloudFormation Generator
//! Pure Rust generator for AWS CloudFormation templates with secrets integration

use serde_json::{json, Value};
use std::collections::HashMap;

fn main() {
    println!("🏛️  MONSTER CLOUDFORMATION GENERATOR");
    println!("===================================");
    
    let template = generate_monster_validator_template();
    
    // Output CloudFormation JSON
    println!("{}", serde_json::to_string_pretty(&template).unwrap());
}

fn generate_monster_validator_template() -> Value {
    json!({
        "AWSTemplateFormatVersion": "2010-09-09",
        "Description": "Monster Group Solana Validator with embedded compiler - 300MB AWS free tier optimized",
        "Parameters": {
            "MonsterFactor": {
                "Type": "Number",
                "Default": 71,
                "Description": "Monster Group alignment factor (sentinel: 71)"
            },
            "MemoryLimitMB": {
                "Type": "Number", 
                "Default": 300,
                "Description": "Memory limit in MB for validator"
            },
            "SSHKeyName": {
                "Type": "AWS::EC2::KeyPair::KeyName",
                "Description": "SSH key pair for instance access"
            },
            "AdminCIDR": {
                "Type": "String",
                "Default": "0.0.0.0/0",
                "Description": "CIDR for admin access to Monster compiler"
            },
            "CompilerSecret": {
                "Type": "String",
                "NoEcho": true,
                "Description": "Secret key for Monster Group compiler access"
            },
            "ValidatorSecret": {
                "Type": "String", 
                "NoEcho": true,
                "Description": "Secret for Solana validator identity"
            }
        },
        "Resources": {
            "MonsterValidatorSecurityGroup": {
                "Type": "AWS::EC2::SecurityGroup",
                "Properties": {
                    "GroupDescription": "Security group for Monster Group Solana validator",
                    "SecurityGroupIngress": [
                        {
                            "IpProtocol": "tcp",
                            "FromPort": 8899,
                            "ToPort": 8899,
                            "CidrIp": "0.0.0.0/0",
                            "Description": "Solana RPC"
                        },
                        {
                            "IpProtocol": "tcp", 
                            "FromPort": 8900,
                            "ToPort": 8900,
                            "CidrIp": "0.0.0.0/0",
                            "Description": "Solana WebSocket"
                        },
                        {
                            "IpProtocol": "tcp",
                            "FromPort": 7171, 
                            "ToPort": 7171,
                            "CidrIp": {"Ref": "AdminCIDR"},
                            "Description": "Monster Group compiler (admin only)"
                        },
                        {
                            "IpProtocol": "tcp",
                            "FromPort": 22,
                            "ToPort": 22, 
                            "CidrIp": {"Ref": "AdminCIDR"},
                            "Description": "SSH access"
                        },
                        {
                            "IpProtocol": "tcp",
                            "FromPort": 4001,
                            "ToPort": 4001,
                            "CidrIp": "0.0.0.0/0", 
                            "Description": "IPFS swarm"
                        }
                    ],
                    "Tags": [
                        {"Key": "Name", "Value": "monster-validator-sg"},
                        {"Key": "Type", "Value": "MonsterGroup"}
                    ]
                }
            },
            "MonsterValidatorRole": {
                "Type": "AWS::IAM::Role",
                "Properties": {
                    "AssumeRolePolicyDocument": {
                        "Version": "2012-10-17",
                        "Statement": [{
                            "Effect": "Allow",
                            "Principal": {"Service": "ec2.amazonaws.com"},
                            "Action": "sts:AssumeRole"
                        }]
                    },
                    "Policies": [{
                        "PolicyName": "MonsterValidatorPolicy",
                        "PolicyDocument": {
                            "Version": "2012-10-17",
                            "Statement": [
                                {
                                    "Effect": "Allow",
                                    "Action": [
                                        "logs:CreateLogGroup",
                                        "logs:CreateLogStream", 
                                        "logs:PutLogEvents"
                                    ],
                                    "Resource": {"Fn::Sub": "arn:aws:logs:${AWS::Region}:${AWS::AccountId}:log-group:/aws/ec2/monster-validator:*"}
                                },
                                {
                                    "Effect": "Allow",
                                    "Action": [
                                        "secretsmanager:GetSecretValue"
                                    ],
                                    "Resource": [
                                        {"Ref": "CompilerSecretStore"},
                                        {"Ref": "ValidatorSecretStore"}
                                    ]
                                }
                            ]
                        }
                    }]
                }
            },
            "MonsterValidatorInstanceProfile": {
                "Type": "AWS::IAM::InstanceProfile",
                "Properties": {
                    "Roles": [{"Ref": "MonsterValidatorRole"}]
                }
            },
            "CompilerSecretStore": {
                "Type": "AWS::SecretsManager::Secret",
                "Properties": {
                    "Name": "monster-compiler-secret",
                    "Description": "Secret for Monster Group compiler access",
                    "SecretString": {"Ref": "CompilerSecret"}
                }
            },
            "ValidatorSecretStore": {
                "Type": "AWS::SecretsManager::Secret", 
                "Properties": {
                    "Name": "monster-validator-secret",
                    "Description": "Secret for Solana validator identity",
                    "SecretString": {"Ref": "ValidatorSecret"}
                }
            },
            "MonsterValidatorLogGroup": {
                "Type": "AWS::Logs::LogGroup",
                "Properties": {
                    "LogGroupName": "/aws/ec2/monster-validator",
                    "RetentionInDays": 7
                }
            },
            "MonsterValidatorInstance": {
                "Type": "AWS::EC2::Instance",
                "Properties": {
                    "ImageId": "ami-0c02fb55956c7d316",
                    "InstanceType": "t2.micro",
                    "KeyName": {"Ref": "SSHKeyName"},
                    "SecurityGroupIds": [{"Ref": "MonsterValidatorSecurityGroup"}],
                    "IamInstanceProfile": {"Ref": "MonsterValidatorInstanceProfile"},
                    "UserData": {
                        "Fn::Base64": {
                            "Fn::Sub": generate_user_data_template()
                        }
                    },
                    "BlockDeviceMappings": [{
                        "DeviceName": "/dev/xvda",
                        "Ebs": {
                            "VolumeType": "gp3",
                            "VolumeSize": 8,
                            "Encrypted": true
                        }
                    }],
                    "Tags": [
                        {"Key": "Name", "Value": "monster-solana-validator"},
                        {"Key": "Type", "Value": "MonsterGroup"},
                        {"Key": "MonsterFactor", "Value": {"Ref": "MonsterFactor"}},
                        {"Key": "MemoryLimit", "Value": {"Fn::Sub": "${MemoryLimitMB}MB"}},
                        {"Key": "Compiler", "Value": "Embedded"},
                        {"Key": "SelfModifying", "Value": "true"}
                    ]
                }
            },
            "MonsterValidatorEIP": {
                "Type": "AWS::EC2::EIP",
                "Properties": {
                    "InstanceId": {"Ref": "MonsterValidatorInstance"},
                    "Tags": [
                        {"Key": "Name", "Value": "monster-validator-eip"},
                        {"Key": "Type", "Value": "MonsterGroup"}
                    ]
                }
            }
        },
        "Outputs": {
            "ValidatorInstanceId": {
                "Description": "Monster validator instance ID",
                "Value": {"Ref": "MonsterValidatorInstance"}
            },
            "ValidatorPublicIP": {
                "Description": "Monster validator public IP",
                "Value": {"Ref": "MonsterValidatorEIP"}
            },
            "SolanaRPCEndpoint": {
                "Description": "Solana RPC endpoint",
                "Value": {"Fn::Sub": "http://${MonsterValidatorEIP}:8899"}
            },
            "MonsterCompilerEndpoint": {
                "Description": "Monster Group compiler endpoint (admin only)",
                "Value": {"Fn::Sub": "http://${MonsterValidatorEIP}:7171"}
            },
            "IPFSEndpoint": {
                "Description": "IPFS agent memory endpoint", 
                "Value": {"Fn::Sub": "http://${MonsterValidatorEIP}:5001"}
            },
            "SSHCommand": {
                "Description": "SSH command to connect",
                "Value": {"Fn::Sub": "ssh -i ~/.ssh/${SSHKeyName}.pem ec2-user@${MonsterValidatorEIP}"}
            }
        }
    })
}

fn generate_user_data_template() -> String {
    r#"#!/bin/bash
# Monster Group Solana Validator CloudFormation Initialization
set -e

MONSTER_FACTOR=${MonsterFactor}
MEMORY_LIMIT=${MemoryLimitMB}
COMPILER_SECRET_ARN=${CompilerSecretStore}
VALIDATOR_SECRET_ARN=${ValidatorSecretStore}

echo "🏛️  MONSTER VALIDATOR CLOUDFORMATION INIT"
echo "Monster Factor: $MONSTER_FACTOR"
echo "Memory Limit: ${MEMORY_LIMIT}MB"

# Install dependencies
yum update -y
yum install -y git curl wget htop awscli

# Retrieve secrets
COMPILER_SECRET=$(aws secretsmanager get-secret-value --secret-id $COMPILER_SECRET_ARN --query SecretString --output text)
VALIDATOR_SECRET=$(aws secretsmanager get-secret-value --secret-id $VALIDATOR_SECRET_ARN --query SecretString --output text)

# Install Nix
curl -L https://nixos.org/nix/install | sh
source ~/.nix-profile/etc/profile.d/nix.sh

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Clone Monster repository
cd /opt
git clone --recursive https://github.com/meta-introspector/cargo2nix.git monster-validator
cd monster-validator

# Build Monster Validator
cd tools/rust-71-parts
cargo build --release --bin solana_validator_demo
cp target/release/solana_validator_demo /usr/local/bin/monster-validator

# Create validator service with secrets
cat > /etc/systemd/system/monster-validator.service << EOF
[Unit]
Description=Monster Group Solana Validator
After=network.target

[Service]
Type=simple
User=ec2-user
WorkingDirectory=/opt/monster-validator
ExecStart=/usr/local/bin/monster-validator --daemon
Restart=always
RestartSec=10
Environment=MONSTER_FACTOR=$MONSTER_FACTOR
Environment=MEMORY_LIMIT_MB=$MEMORY_LIMIT
Environment=COMPILER_SECRET=$COMPILER_SECRET
Environment=VALIDATOR_SECRET=$VALIDATOR_SECRET
MemoryMax=${MEMORY_LIMIT}M

[Install]
WantedBy=multi-user.target
EOF

# Create Monster compiler service (hidden)
cat > /etc/systemd/system/monster-compiler.service << EOF
[Unit]
Description=Monster Group Embedded Compiler
After=monster-validator.service

[Service]
Type=simple
User=ec2-user
WorkingDirectory=/opt/monster-validator
ExecStart=/usr/local/bin/monster-compiler --port 7171 --secret $COMPILER_SECRET
Restart=always
RestartSec=5
Environment=MONSTER_FACTOR=$MONSTER_FACTOR
MemoryMax=50M

[Install]
WantedBy=multi-user.target
EOF

# Configure CloudWatch agent
yum install -y amazon-cloudwatch-agent
cat > /opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json << EOF
{
    "logs": {
        "logs_collected": {
            "files": {
                "collect_list": [{
                    "file_path": "/var/log/monster-validator.log",
                    "log_group_name": "/aws/ec2/monster-validator",
                    "log_stream_name": "validator-{instance_id}"
                }]
            }
        }
    }
}
EOF

/opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl \
    -a fetch-config -m ec2 -c file:/opt/aws/amazon-cloudwatch-agent/etc/amazon-cloudwatch-agent.json -s

# Start services
systemctl daemon-reload
systemctl enable monster-validator monster-compiler
systemctl start monster-validator
sleep 5
systemctl start monster-compiler

echo "✅ Monster Validator CloudFormation deployment complete!"
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_template_generation() {
        let template = generate_monster_validator_template();
        assert!(template["Resources"]["MonsterValidatorInstance"].is_object());
        assert_eq!(template["Parameters"]["MonsterFactor"]["Default"], 71);
    }
}
