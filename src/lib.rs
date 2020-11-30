

mod logentry;


pub enum NodeState {
    Leader,
    Candidate,
    Follower
}


pub struct RaftNode {
    
    id: usize,                          // The ID for this node
    state: NodeState,                   // Every node is in one of three states: this keeps track of which state it is in.

    // Persistent state is updated on stable storage (a log file???) before responding to RPCs
    current_term: usize,                // Persistent, latest term server has seen (initialized to 0 on first boot, increases monotonically)
    votedFor: Option<usize>,            // Persistent, candidate_id that received vote in current term (or None if none)
    log: Vec<logentry::LogEntry>,       // Persistent, log entries, each entry contains command for state machine, and the term when the entry was received by leader (first index is 1)

    // Volatile state on all servers
    commit_index: usize,                // Index of the highest log entry known to be committed (initialized to 0, increases monotonically)
    last_applied: usize,                // Index of the highest log entry applied to state machine (initialized to 0, increases monotonically)

    // Volatile state on leaders (i.e. reinitialized after election)
    next_index: Vec<usize>,             // For each server, index of the next log entry to send to that server (initialized to leader last log index + 1)
    match_index: Vec<usize>             // For each server, index of the highest log entry known to be replicated on server

}


impl RaftNode {

    
}