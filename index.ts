import { Horizon, rpc, xdr, Networks, Operation, Address, TransactionBuilder, authorizeEntry, Keypair } from '@stellar/stellar-sdk/minimal';
import { assembleTransaction, Api } from '@stellar/stellar-sdk/minimal/rpc';

const networkPassphrase = Networks.TESTNET;
const contractId = 'CCBD7MSVDXIKXY66YXQQQZ4ENWAF6CF5ZY3SSJBS6E2TN24SZRVX7XPP';
const sac = 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC';

const rpcUrl = 'https://soroban-testnet.stellar.org';
const rpcServer = new rpc.Server(rpcUrl);

const horizonUrl = 'https://horizon-testnet.stellar.org';
const horizonServer = new Horizon.Server(horizonUrl);

const keypair = Keypair.fromSecret('SAXLQLVOTCTEL3PQG6NPGPMQXTZE66V7BSKAHQZFBEQGOSMEYMPYL5KS');
const source = keypair.publicKey();

// Build a transfer op
const transfer_op = Operation.invokeContractFunction({
    contract: sac,
    function: 'transfer',
    args: [
        Address.fromString(source).toScVal(),
        Address.fromString(source).toScVal(),
        xdr.ScVal.scvI128(new xdr.Int128Parts({ 
            hi: new xdr.Int64(0),
            lo: new xdr.Uint64(100),
        })),
    ]
});

const transfer_txn = new TransactionBuilder(
    await rpcServer.getAccount(source), 
    {
        fee: '0',
        networkPassphrase,
    }
)
.addOperation(transfer_op)
.setTimeout(0)
.build();

// Simulate to get the auth entry
const transfer_sim = await rpcServer.simulateTransaction(transfer_txn);

if (!Api.isSimulationSuccess(transfer_sim)) {
    throw new Error('Simulation failed');
}

const { sequence } = await rpcServer.getLatestLedger()

let transfer_auth;

// Sign the auth entry
for (const auth of transfer_sim.result?.auth || []) {
    transfer_auth = await authorizeEntry(
        auth,
        keypair,
        sequence + 1000,
        networkPassphrase
    );
}

// Build the call op with the transfer auth
const call_op = Operation.invokeContractFunction({
    contract: contractId,
    function: 'call',
    args: [
        Address.fromString(source).toScVal(),
        Address.fromString(sac).toScVal(),
    ],
    auth: [transfer_auth!]
})

const call_txn_pre = new TransactionBuilder(
    await horizonServer.loadAccount(source), 
    {
        fee: '100',
        networkPassphrase,
    }
)
.addOperation(call_op)
.setTimeout(30)
.build();

// Simulate the call txn
const call_sim = await rpcServer.simulateTransaction(call_txn_pre);

if (!Api.isSimulationSuccess(call_sim)) {
    throw new Error('Simulation failed');
}

// Assemble and sign the call txn
const call_txn = assembleTransaction(call_txn_pre, call_sim).build();

call_txn.sign(keypair);

const res_send = await rpcServer._sendTransaction(call_txn);

console.log(res_send);
console.log(call_txn.toXDR());

if (res_send.status === 'PENDING') {
    const res_get = await rpcServer.pollTransaction(res_send.hash);
    console.log(res_get.txHash);
}