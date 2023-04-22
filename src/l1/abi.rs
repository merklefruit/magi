use ethers::prelude::abigen;

abigen!(
    OptimismPortal,
    r#"[
        function GUARDIAN() external view returns (address)
        function L2_ORACLE() external view returns (address)
        function SYSTEM_CONFIG() external view returns (address)
    ]"#
);

abigen!(
    L2OutputOracle,
    r#"[
        function SUBMISSION_INTERVAL() external view returns (uint256)
        function L2_BLOCK_TIME() external view returns (uint256)
        function CHALLENGER() external view returns (address)
        function PROPOSER() external view returns (address)
        function FINALIZATION_PERIOD_SECONDS() external view returns (uint256)
        function startingBlockNumber() external view returns (uint256)
        function startingTimestamp() external view returns (uint256)
        function getL2Output(uint256 _l2OutputIndex) external view returns (uint256)
        function getL2OutputIndexAfter(uint256 _l2BlockNumber) external view returns (uint256)

        event OutputProposed(bytes32 indexed outputRoot, uint256 indexed l2OutputIndex, uint256 indexed l2BlockNumber,uint256 l1Timestamp)
        
        struct OutputProposal { bytes32 outputRoot; uint128 timestamp; uint128 l2BlockNumber; }
    ]"#
);
