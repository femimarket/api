import Foundation
@testable import Api

/// Fresh per-run credentials for live-server tests. The server auto-funds any new
/// user with 50 credits, so a random username + arbitrary password gets us a funded
/// account for the duration of the test process.
let testUser = "funded-test-\(UUID().uuidString)"
let testPassword = "abc123"
