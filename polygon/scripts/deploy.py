'''
Scripts to deploy the smart contract to Mumbai:
brownie run deploy.py --network polygon-test
'''
from brownie import StoreScores, accounts, config
from dotenv import load_dotenv
load_dotenv()

def main():
    '''
    Load your Infura account and deploy the contract from that account
    '''
    acc = accounts.add(config["wallets"]["from_key"])
    StoreScores.deploy({"from": acc}, publish_source=True)