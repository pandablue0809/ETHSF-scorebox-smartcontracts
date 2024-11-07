pragma solidity ^0.8.17;
pragma experimental ABIEncoderV2;
// SPDX-License-Identifier: GPL-3.0

import "../node_modules/@openzeppelin/contracts/access/Ownable.sol";


//contract
contract StoreScores is Ownable {
    struct User {
        int16 score;
        string description;
    }

    //define members for your smart contract
    mapping(address => User[]) public records;
    uint256 public userCount;
    uint256 public scoreCount;

    event UploadSucceeded(address wallet, bool outcome);
    error NoScoreHistory(address wallet);
    
    constructor() Ownable() {
        userCount = 0;
        scoreCount = 0;
    }


    //require attached fee for function call
    modifier requiresFee(uint fee) {
        require(
            msg.value == fee,
            "INSUFFICIENT_FUNDS"
            );
        _ ;
    }

    //save credit scores on blockchain
    function uploadScore(
        int16 _score,
        string calldata _description,
        address _beneficiary,
        uint256 _amount
    ) requiresFee(_amount) public payable returns (bool b) {
        b = false;

        if (records[msg.sender].length != 0) {
            //if it's a returning user, append new score to existing mapping
            records[msg.sender].push(User(_score, _description));
            scoreCount++;
            b = true;
        } else {
            //if it's a new user, add new score entry to mapping
            User[] storage x = records[msg.sender];
            x.push(User(_score, _description));
            userCount++;
            scoreCount++;
            b = true;
        }
        (bool success, ) = payable(_beneficiary).call{value: _amount}("");
        require(success, "FAILED_TO_PAY_SERVICE_CHARGE");

        emit UploadSucceeded(msg.sender, b);
        return b;
    }

    //query score history
    function getScore(address _wallet) public view returns (User[] memory r) {
        if (records[_wallet].length == 0) revert NoScoreHistory(_wallet);
        r = records[_wallet];
        r;
    }
}