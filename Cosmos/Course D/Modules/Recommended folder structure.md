

A typical Cosmos SDK module can be structured as follows:

1.  The serializable data types and Protobuf interfaces
  ```
  proto
└── {project_name}
    └── {module_name}
        └── {proto_version}
            ├── {module_name}.proto
            ├── event.proto
            ├── genesis.proto
            ├── query.proto
            └── tx.proto

  ```