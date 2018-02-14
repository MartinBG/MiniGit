//arguments: commitId (will be hash of commit)
// change directory as specified in commit info
// cannot commit if not head

//steps:
//1)load commits: destination and current
	// -deserialize commit
	// -get head commit path
//2)Get differences
// 3) Add and delete files