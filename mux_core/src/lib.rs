// type ReplicaId = UUID;
type LocalTimestamp = usize;

struct WorkTreeId {
    replica_id: ReplicaId,
    timestamp: LocalTimestamp,
}

struct Repository {
    local_clock: LocalTimestamp,
    replica_id: ReplicaId,
    work_trees: HashMap<WorkTreeId, WorkTree>,
}

struct WorkTree {
    nodes: HashMap<NodeId, Rc<RefCell<Node>>>,
    root: Node,
    epochs: HashMap<EpochId, Option<CommitSha>>,
}

struct OpId {
    work_tree_id:
    epoch_id: EpochId,
    replica_id: ReplicaId,
    timestamp: LocalTimestamp,
}

struct Operation {
    id: OpId,
    type: OperationType,
}

enum OperationType {
    CreateEpoch(Epoch),
    MoveNode {
        node_id: NodeId,
        new_parent_id: NodeId,
    },
    RenameNode {
        node_id: NodeId,
        name: String,
    }
}

struct Register<T> {
    value: T,
    last_replica_id: ReplicaId,
    last_timestamp: LamportTimestamp,
}

struct Node {
    id: OpId,
    name: Register<String>,
    alternate_name: Option<String>,
    parent_id: Register<NodeId>,
    type: NodeType,
}

enum NodeType {
    Directory {
        children: BTreeMap<String, Vec<Rc<RefCell<Node>>>>
    },
    File {
        contents: FileContents
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
