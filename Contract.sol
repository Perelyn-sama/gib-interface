// SPDX-License-Identifier: MIT
pragma solidity >=0.8.19;

interface IContract {
function tokenURI(IERC721Metadata sablier, uint256 streamId) external view returns (string memory uri);function abbreviateAmount(uint256 amount, uint256 decimals) internal pure returns (string memory);function calculateDurationInDays(uint256 startTime, uint256 endTime) internal pure returns (string memory);function calculateStreamedPercentage(uint128 streamedAmount, uint128 depositedAmount);
}