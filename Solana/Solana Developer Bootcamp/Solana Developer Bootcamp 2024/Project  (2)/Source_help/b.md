<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Solana Voting Contract Mind Map</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 20px;
            padding: 30px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
        }
        h1 {
            text-align: center;
            color: #2c3e50;
            margin-bottom: 30px;
            font-size: 2.5em;
        }
        .mindmap {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 30px;
        }
        .central-node {
            background: linear-gradient(45deg, #FF6B6B, #4ECDC4);
            color: white;
            padding: 20px 40px;
            border-radius: 50px;
            font-size: 1.5em;
            font-weight: bold;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
            text-align: center;
        }
        .branches {
            display: flex;
            justify-content: space-around;
            width: 100%;
            flex-wrap: wrap;
            gap: 20px;
        }
        .branch {
            flex: 1;
            min-width: 300px;
            background: #f8f9fa;
            border-radius: 15px;
            padding: 20px;
            box-shadow: 0 5px 15px rgba(0,0,0,0.1);
            transition: transform 0.3s ease;
        }
        .branch:hover {
            transform: translateY(-5px);
        }
        .branch-title {
            font-size: 1.3em;
            font-weight: bold;
            color: #2c3e50;
            margin-bottom: 15px;
            text-align: center;
            padding: 10px;
            border-radius: 10px;
        }
        .functions { background: linear-gradient(45deg, #667eea, #764ba2); color: white; }
        .accounts { background: linear-gradient(45deg, #f093fb, #f5576c); color: white; }
        .data { background: linear-gradient(45deg, #4facfe, #00f2fe); color: white; }
        .errors { background: linear-gradient(45deg, #fa709a, #fee140); color: white; }
        .config { background: linear-gradient(45deg, #a8edea, #fed6e3); color: #2c3e50; }
        
        .sub-items {
            list-style: none;
            padding: 0;
            margin: 0;
        }
        .sub-items li {
            background: white;
            margin: 10px 0;
            padding: 15px;
            border-radius: 10px;
            border-left: 4px solid #3498db;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            transition: all 0.3s ease;
        }
        .sub-items li:hover {
            background: #ecf0f1;
            border-left-color: #e74c3c;
        }
        .code-snippet {
            background: #2c3e50;
            color: #ecf0f1;
            padding: 5px 10px;
            border-radius: 5px;
            font-family: 'Courier New', monospace;
            font-size: 0.9em;
            margin: 5px 0;
        }
        .connection-line {
            width: 2px;
            height: 30px;
            background: linear-gradient(to bottom, #3498db, #9b59b6);
            margin: 0 auto;
        }
        .key-concepts {
            background: #fff3cd;
            border: 2px solid #ffeaa7;
            border-radius: 15px;
            padding: 20px;
            margin: 20px 0;
        }
        .key-concepts h3 {
            color: #d63031;
            margin-top: 0;
        }
        .comparison {
            background: #d4edda;
            border: 2px solid #95db95;
            border-radius: 15px;
            padding: 20px;
            margin: 20px 0;
        }
        .comparison h3 {
            color: #155724;
            margin-top: 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🗳️ Solana Voting Contract Architecture</h1>
        
        <div class="key-concepts">
            <h3>🔑 Key Concepts for Solidity Developers</h3>
            <p><strong>Account Model:</strong> Unlike Solidity's storage variables, Solana uses separate accounts to store data</p>
            <p><strong>Seeds & Bumps:</strong> Deterministic account addresses (like CREATE2 in Solidity)</p>
            <p><strong>Context:</strong> Contains all accounts and metadata needed for the instruction</p>
            <p><strong>Anchor Framework:</strong> Similar to Hardhat/Foundry - provides structure and safety</p>
        </div>

        <div class="mindmap">
            <div class="central-node">
                Solana Voting Contract<br>
                <div class="code-snippet">declare_id!("5s3PtT8kLYCv1WEp6dSh3T7EuF35Z6jSu5Cvx4hWG79H")</div>
            </div>

            <div class="connection-line"></div>

            <div class="branches">
                <div class="branch">
                    <div class="branch-title functions">📋 Program Functions</div>
                    <ul class="sub-items">
                        <li>
                            <strong>initialize_poll</strong>
                            <div class="code-snippet">ctx: Context&lt;InitializePoll&gt;</div>
                            Creates a new poll with name, description, and voting window
                        </li>
                        <li>
                            <strong>initialize_candidate</strong>
                            <div class="code-snippet">ctx: Context&lt;InitializeCandidate&gt;</div>
                            Adds a candidate to an existing poll
                        </li>
                        <li>
                            <strong>vote</strong>
                            <div class="code-snippet">ctx: Context&lt;Vote&gt;</div>
                            Records a vote for a candidate (with time validation)
                        </li>
                    </ul>
                </div>

                <div class="branch">
                    <div class="branch-title accounts">🏦 Account Structures</div>
                    <ul class="sub-items">
                        <li>
                            <strong>InitializePoll</strong>
                            <div class="code-snippet">seeds = [b"poll", poll_id]</div>
                            Signer + PollAccount + SystemProgram
                        </li>
                        <li>
                            <strong>InitializeCandidate</strong>
                            <div class="code-snippet">seeds = [poll_id, candidate]</div>
                            Signer + PollAccount + CandidateAccount
                        </li>
                        <li>
                            <strong>Vote</strong>
                            <div class="code-snippet">mut poll_account, mut candidate_account</div>
                            Signer + PollAccount + CandidateAccount
                        </li>
                    </ul>
                </div>

                <div class="branch">
                    <div class="branch-title data">💾 Data Structures</div>
                    <ul class="sub-items">
                        <li>
                            <strong>PollAccount</strong>
                            <div class="code-snippet">#[derive(InitSpace)]</div>
                            poll_name, poll_description, start/end times, option_index
                        </li>
                        <li>
                            <strong>CandidateAccount</strong>
                            <div class="code-snippet">#[max_len(32)]</div>
                            candidate_name, candidate_votes counter
                        </li>
                    </ul>
                </div>

                <div class="branch">
                    <div class="branch-title errors">⚠️ Error Handling</div>
                    <ul class="sub-items">
                        <li>
                            <strong>VotingNotStarted</strong>
                            <div class="code-snippet">current_time &lt;= start_time</div>
                            Prevents voting before poll opens
                        </li>
                        <li>
                            <strong>VotingEnded</strong>
                            <div class="code-snippet">current_time &gt; end_time</div>
                            Prevents voting after poll closes
                        </li>
                    </ul>
                </div>

                <div class="branch">
                    <div class="branch-title config">⚙️ Configuration</div>
                    <ul class="sub-items">
                        <li>
                            <strong>Account Constraints</strong>
                            <div class="code-snippet">init_if_needed, payer = signer</div>
                            Automatic account creation and rent payment
                        </li>
                        <li>
                            <strong>Space Allocation</strong>
                            <div class="code-snippet">space = 8 + INIT_SPACE</div>
                            Deterministic storage size calculation
                        </li>
                        <li>
                            <strong>Seeds & Bumps</strong>
                            <div class="code-snippet">seeds = [...], bump</div>
                            Deterministic account address generation
                        </li>
                    </ul>
                </div>
            </div>
        </div>

        <div class="comparison">
            <h3>🔄 Solidity vs Solana Comparison</h3>
            <p><strong>Solidity:</strong> Single contract with storage variables → <strong>Solana:</strong> Multiple accounts with individual data</p>
            <p><strong>Solidity:</strong> msg.sender → <strong>Solana:</strong> ctx.accounts.signer</p>
            <p><strong>Solidity:</strong> require() statements → <strong>Solana:</strong> Custom error enums</p>
            <p><strong>Solidity:</strong> Constructor → <strong>Solana:</strong> Initialize functions with account creation</p>
        </div>
    </div>
</body>
</html>