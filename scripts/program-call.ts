import {
    Account,
    Connection,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction
} from '@solana/web3.js'
import {Token} from '@solana/spl-token'
import * as fs from "fs";
import * as bip39 from "bip39";
import {derivePath} from "ed25519-hd-key";
import * as nacl from "tweetnacl";



// variables
const mnemonic = fs.readFileSync("./test-account-mnenomic.txt").toString().trim();
const conn = new Connection("https://devnet.solana.com")
const programId = new PublicKey("2zq2h26QcqsUD8jjrXUA83iVgmSzibzVA9XThU5vB99i")
main()

async function main() {
    let acc = await createAccountFromMnemonic(mnemonic)

    let instranction = new TransactionInstruction({
        keys: [{
            pubkey: acc.publicKey,
            isWritable: false,
            isSigner: true},
            ],
        programId:programId,
        data:new Buffer(8),
    })

    let tx = new Transaction()

    tx.add(instranction)

    let txHash = await sendAndConfirmTransaction(conn,tx,[acc])

    console.log(txHash)
}


/**
 * Create an Account
 */
async function createNewAccount() {
    const mnemonic =  await bip39.generateMnemonic(256)
    let account = await createAccountFromMnemonic(mnemonic)
    return account
}


/**
 * Create an Account from Mnemonic
 */
async function createAccountFromMnemonic(mnemonic) {
    console.log("\n================== CreateAccount From Mnemonic==========================")
    console.log(mnemonic)

    let seed = await bip39.mnemonicToSeed(mnemonic)
    let seedHex = Buffer.from(seed).toString('hex')
    const derivedSeed = derivePath(`m/44'/501'/0'/0'`, seedHex).key
    let account = new Account(nacl.sign.keyPair.fromSeed(derivedSeed).secretKey)

    console.log("\nPublic Key(Address):",account.publicKey.toBase58())
    console.log("===========================================\n")

    return account
}

