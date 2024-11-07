# :rocket: ScoreBox Contract | Polygon
This is a smart contract written in Solidity and deployed on Polygon testnet (Mumbai), which can store users' credit scores and return them for free when queried using a view call.
___

### Core functions
Notice the first is a payable function, whereas the second is just a viewing function
```bash
    #save scores
    function uploadScore(
        int16 _score,
        string _description,
        address _beneficiary,
        uint256 _amount
    ) requiresFee(_amount) returns (bool b)

    # query scores
    function getScore(address _wallet) 
        returns (User[]) # returns a vector of Structs
```
___

### Solidity env set up
```bash
npm install --save-dev "hardhat@^2.12.0"
npm install @openzeppelin/contracts
```

### Test & compile
You need Brownie, so install it. </br>
Good alternatives are HardHat and Truffle, but both are JS based.
```bash
pip install eth-brownie
```

### Deploy
Using Brownie
```bash
brownie compile
brownie run deploy.py --network polygon-test
```


### Interact
Run from terminal
```bash
# import accounts
brownie accounts list
brownie accounts new josi  # enter josi's private key and a password
brownie accounts list


# connect to blockchain
brownie console --network polygon-test                                
    network.is_connected()                                             
    network.show_active()                                              


# interact with contract
    sc = StoreScores.at('yourcontractaddress')                        
    nene = accounts.load('nene')                                      
    josi = accounts.load('josi')                                     
    sc.userCount()                                                   
    sc.uploadScore(501, 'Yo!', nene, 3, {'from':josi, 'value': 3}) 
    sc.getScore(josi, {'from': josi})                                  
```

> `nene` and `josi` are an alias
