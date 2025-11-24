//! Monster Group Solana Validator OCI Terraform Generator
//! Pure Rust generator using terrars for Oracle Cloud Infrastructure

use serde_json::{json, Value};

fn main() {
    println!("🏛️  MONSTER OCI TERRAFORM GENERATOR");
    println!("==================================");
    
    let terraform = generate_monster_oci_terraform();
    
    // Output Terraform JSON
    println!("{}", serde_json::to_string_pretty(&terraform).unwrap());
}

fn generate_monster_oci_terraform() -> Value {
    json!({
        "terraform": {
            "required_providers": {
                "oci": {
                    "source": "oracle/oci",
                    "version": "~> 5.0"
                }
            }
        },
        "provider": {
            "oci": {
                "tenancy_ocid": "${var.tenancy_ocid}",
                "user_ocid": "${var.user_ocid}",
                "fingerprint": "${var.fingerprint}",
                "private_key_path": "${var.private_key_path}",
                "region": "${var.region}"
            }
        },
        "variable": {
            "tenancy_ocid": {"type": "string"},
            "user_ocid": {"type": "string"},
            "fingerprint": {"type": "string"},
            "private_key_path": {"type": "string"},
            "region": {"type": "string", "default": "us-ashburn-1"},
            "compartment_ocid": {"type": "string"},
            "monster_factor": {"type": "number", "default": 71},
            "memory_limit_mb": {"type": "number", "default": 300},
            "ssh_public_key": {"type": "string"},
            "compiler_secret": {"type": "string", "sensitive": true},
            "validator_secret": {"type": "string", "sensitive": true}
        },
        "data": {
            "oci_identity_availability_domains": {
                "ads": {
                    "compartment_id": "${var.tenancy_ocid}"
                }
            },
            "oci_core_images": {
                "oracle_linux": {
                    "compartment_id": "${var.compartment_ocid}",
                    "operating_system": "Oracle Linux",
                    "operating_system_version": "8",
                    "shape": "VM.Standard.E2.1.Micro"
                }
            }
        },
        "resource": {
            "oci_kms_vault": {
                "monster_vault": {
                    "compartment_id": "${var.compartment_ocid}",
                    "display_name": "monster-validator-vault",
                    "vault_type": "DEFAULT",
                    "freeform_tags": {
                        "Type": "MonsterGroup",
                        "Component": "Vault"
                    }
                }
            },
            "oci_kms_key": {
                "monster_key": {
                    "compartment_id": "${var.compartment_ocid}",
                    "display_name": "monster-validator-key",
                    "management_endpoint": "${oci_kms_vault.monster_vault.management_endpoint}",
                    "key_shape": {
                        "algorithm": "AES",
                        "length": 32
                    },
                    "freeform_tags": {
                        "Type": "MonsterGroup",
                        "Component": "EncryptionKey"
                    }
                }
            },
            "oci_vault_secret": {
                "compiler_secret": {
                    "compartment_id": "${var.compartment_ocid}",
                    "vault_id": "${oci_kms_vault.monster_vault.id}",
                    "key_id": "${oci_kms_key.monster_key.id}",
                    "secret_name": "monster-compiler-secret",
                    "secret_content": {
                        "content_type": "BASE64",
                        "content": "${base64encode(var.compiler_secret)}"
                    },
                    "freeform_tags": {
                        "Type": "MonsterGroup",
                        "Component": "CompilerSecret"
                    }
                }
            },
            "oci_vault_secret": {
                "validator_secret": {
                    "compartment_id": "${var.compartment_ocid}",
                    "vault_id": "${oci_kms_vault.monster_vault.id}",
                    "key_id": "${oci_kms_key.monster_key.id}",
                    "secret_name": "monster-validator-secret",
                    "secret_content": {
                        "content_type": "BASE64",
                        "content": "${base64encode(var.validator_secret)}"
                    },
                    "freeform_tags": {
                        "Type": "MonsterGroup",
                        "Component": "ValidatorSecret"
                    }
                }
            },
            "oci_core_vcn": {
                "monster_vcn": {
                    "compartment_id": "${var.compartment_ocid}",
                    "cidr_blocks": ["10.0.0.0/16"],
                    "display_name": "monster-validator-vcn",
                    "freeform_tags": {
                        "Type": "MonsterGroup",
                        "Component": "Network"
                    }
                }
            },
            "oci_core_internet_gateway": {
                "monster_igw": {
                    "compartment_id": "${var.compartment_ocid}",
                    "vcn_id": "${oci_core_vcn.monster_vcn.id}",
                    "display_name": "monster-validator-igw"
                }
            },
            "oci_core_route_table": {
                "monster_rt": {
                    "compartment_id": "${var.compartment_ocid}",
                    "vcn_id": "${oci_core_vcn.monster_vcn.id}",
                    "display_name": "monster-validator-rt",
                    "route_rules": [{
                        "destination": "0.0.0.0/0",
                        "network_entity_id": "${oci_core_internet_gateway.monster_igw.id}"
                    }]
                }
            },
            "oci_core_security_list": {
                "monster_sl": {
                    "compartment_id": "${var.compartment_ocid}",
                    "vcn_id": "${oci_core_vcn.monster_vcn.id}",
                    "display_name": "monster-validator-sl",
                    "ingress_security_rules": [
                        {
                            "protocol": "6",
                            "source": "0.0.0.0/0",
                            "tcp_options": {
                                "min": 8899,
                                "max": 8899
                            },
                            "description": "Solana RPC"
                        },
                        {
                            "protocol": "6",
                            "source": "0.0.0.0/0",
                            "tcp_options": {
                                "min": 8900,
                                "max": 8900
                            },
                            "description": "Solana WebSocket"
                        },
                        {
                            "protocol": "6",
                            "source": "0.0.0.0/0",
                            "tcp_options": {
                                "min": 7171,
                                "max": 7171
                            },
                            "description": "Monster Compiler"
                        },
                        {
                            "protocol": "6",
                            "source": "0.0.0.0/0",
                            "tcp_options": {
                                "min": 22,
                                "max": 22
                            },
                            "description": "SSH"
                        },
                        {
                            "protocol": "6",
                            "source": "0.0.0.0/0",
                            "tcp_options": {
                                "min": 4001,
                                "max": 4001
                            },
                            "description": "IPFS"
                        }
                    ],
                    "egress_security_rules": [{
                        "protocol": "all",
                        "destination": "0.0.0.0/0"
                    }]
                }
            },
            "oci_core_subnet": {
                "monster_subnet": {
                    "compartment_id": "${var.compartment_ocid}",
                    "vcn_id": "${oci_core_vcn.monster_vcn.id}",
                    "cidr_block": "10.0.1.0/24",
                    "display_name": "monster-validator-subnet",
                    "route_table_id": "${oci_core_route_table.monster_rt.id}",
                    "security_list_ids": ["${oci_core_security_list.monster_sl.id}"]
                }
            },
            "oci_core_instance": {
                "monster_validator": {
                    "compartment_id": "${var.compartment_ocid}",
                    "availability_domain": "${data.oci_identity_availability_domains.ads.availability_domains[0].name}",
                    "shape": "VM.Standard.E2.1.Micro",
                    "display_name": "monster-solana-validator",
                    "source_details": {
                        "source_type": "image",
                        "source_id": "${data.oci_core_images.oracle_linux.images[0].id}"
                    },
                    "create_vnic_details": {
                        "subnet_id": "${oci_core_subnet.monster_subnet.id}",
                        "assign_public_ip": true
                    },
                    "metadata": {
                        "ssh_authorized_keys": "${var.ssh_public_key}",
                        "user_data": "${base64encode(templatefile(\"${path.module}/user_data_oci.sh\", {\n          monster_factor = var.monster_factor,\n          memory_limit = var.memory_limit_mb,\n          compiler_secret_ocid = oci_vault_secret.compiler_secret.id,\n          validator_secret_ocid = oci_vault_secret.validator_secret.id,\n          region = var.region\n        }))}"
                    },
                    "freeform_tags": {
                        "Type": "MonsterGroup",
                        "MonsterFactor": "${var.monster_factor}",
                        "MemoryLimit": "${var.memory_limit_mb}MB",
                        "Compiler": "Embedded",
                        "SelfModifying": "true"
                    }
                }
            }
        },
        "output": {
            "validator_instance_id": {
                "value": "${oci_core_instance.monster_validator.id}"
            },
            "validator_public_ip": {
                "value": "${oci_core_instance.monster_validator.public_ip}"
            },
            "solana_rpc_endpoint": {
                "value": "http://${oci_core_instance.monster_validator.public_ip}:8899"
            },
            "monster_compiler_endpoint": {
                "value": "http://${oci_core_instance.monster_validator.public_ip}:7171",
                "sensitive": true
            },
            "ipfs_endpoint": {
                "value": "http://${oci_core_instance.monster_validator.public_ip}:5001"
            },
            "ssh_command": {
                "value": "ssh opc@${oci_core_instance.monster_validator.public_ip}"
            },
            "vault_id": {
                "value": "${oci_kms_vault.monster_vault.id}"
            },
            "secrets": {
                "value": {
                    "compiler_secret_id": "${oci_vault_secret.compiler_secret.id}",
                    "validator_secret_id": "${oci_vault_secret.validator_secret.id}"
                },
                "sensitive": true
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oci_terraform_generation() {
        let terraform = generate_monster_oci_terraform();
        assert!(terraform["resource"]["oci_core_instance"]["monster_validator"].is_object());
        assert!(terraform["resource"]["oci_kms_vault"]["monster_vault"].is_object());
    }
}
