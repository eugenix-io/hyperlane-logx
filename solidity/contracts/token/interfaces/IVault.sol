// SPDX-License-Identifier: MIT

pragma solidity >=0.8.19;

interface IVault {
    function depositCollateral(bytes32 subAccountId, uint256 amount, uint256 sourceChainId) external;
}