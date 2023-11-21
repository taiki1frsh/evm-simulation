use ethers::{
    prelude::Lazy,
    types::{Address, Bytes, U256, U64},
};
use std::str::FromStr;

pub type SwappedAmount = U256;
pub type TransferredAmount = U256; 

pub static WEI: Lazy<U256> = Lazy::new(|| U256::from(10).pow(U256::from(18)));
pub static GWEI: Lazy<U256> = Lazy::new(|| U256::from(10).pow(U256::from(9)));

pub static ZERO_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x0000000000000000000000000000000000000000").unwrap());

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap()
}

#[derive(Debug, Clone)]
pub struct Env {
    pub https_url: String,
    pub wss_url: String,
    pub chain_id: U64,
}

impl Env {
    pub fn new() -> Self {
        Env {
            https_url: get_env("HTTPS_URL"),
            wss_url: get_env("WSS_URL"),
            chain_id: U64::from_str(&get_env("CHAIN_ID")).unwrap(),
        }
    }
}

pub static SIMULATOR_CODE: Lazy<Bytes> = Lazy::new(|| {
    // "0x608060405234801561001057600080fd5b50600436106100415760003560e01c8063054d50d41461004657806364bfce6f1461006c578063cf62f25b14610094575b600080fd5b61005961005436600461097d565b6100b1565b6040519081526020015b60405180910390f35b61007f61007a3660046109d2565b610231565b60408051928352602083019190915201610063565b61009c600a81565b60405163ffffffff9091168152602001610063565b6000808411610147576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f556e697377617056324c6962726172793a20494e53554646494349454e545f4960448201527f4e5055545f414d4f554e5400000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6000831180156101575750600082115b6101e3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602860248201527f556e697377617056324c6962726172793a20494e53554646494349454e545f4c60448201527f4951554944495459000000000000000000000000000000000000000000000000606482015260840161013e565b60006101f1856103e5610a4e565b905060006101ff8483610a4e565b9050600082610210876103e8610a4e565b61021a9190610a65565b90506102268183610a78565b979650505050505050565b60008061025573ffffffffffffffffffffffffffffffffffffffff851686886106a8565b6000806000808873ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156102a6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102ca9190610ad1565b506dffffffffffffffffffffffffffff1691506dffffffffffffffffffffffffffff1691508673ffffffffffffffffffffffffffffffffffffffff168873ffffffffffffffffffffffffffffffffffffffff16101561032e57819350809250610335565b8093508192505b50506040517f70a0823100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff888116600483015260009184918916906370a0823190602401602060405180830381865afa1580156103a8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103cc9190610b21565b6103d69190610b3a565b6040517f054d50d4000000000000000000000000000000000000000000000000000000008152600481018290526024810185905260448101849052909150309063054d50d490606401602060405180830381865afa15801561043c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104609190610b21565b9450606461046f600a82610b4d565b61047f9063ffffffff1687610a4e565b6104899190610a78565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015290955060009073ffffffffffffffffffffffffffffffffffffffff8816906370a0823190602401602060405180830381865afa1580156104f9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061051d9190610b21565b90506000808873ffffffffffffffffffffffffffffffffffffffff168a73ffffffffffffffffffffffffffffffffffffffff161061055d57876000610561565b6000885b604080516000815260208101918290527f022c0d9f00000000000000000000000000000000000000000000000000000000909152919350915073ffffffffffffffffffffffffffffffffffffffff8c169063022c0d9f906105cb9085908590309060248101610b95565b600060405180830381600087803b1580156105e557600080fd5b505af11580156105f9573d6000803e3d6000fd5b50506040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015285925073ffffffffffffffffffffffffffffffffffffffff8c1691506370a0823190602401602060405180830381865afa158015610669573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061068d9190610b21565b6106979190610b3a565b965050505050505094509492505050565b6040805173ffffffffffffffffffffffffffffffffffffffff8416602482015260448082018490528251808303909101815260649091019091526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fa9059cbb0000000000000000000000000000000000000000000000000000000017905261073590849061073a565b505050565b600061075c73ffffffffffffffffffffffffffffffffffffffff8416836107d0565b9050805160001415801561078157508080602001905181019061077f9190610c11565b155b15610735576040517f5274afe700000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8416600482015260240161013e565b60606107de838360006107e7565b90505b92915050565b606081471015610825576040517fcd78605900000000000000000000000000000000000000000000000000000000815230600482015260240161013e565b6000808573ffffffffffffffffffffffffffffffffffffffff16848660405161084e9190610c33565b60006040518083038185875af1925050503d806000811461088b576040519150601f19603f3d011682016040523d82523d6000602084013e610890565b606091505b50915091506108a08683836108ac565b925050505b9392505050565b6060826108c1576108bc8261093b565b6108a5565b81511580156108e5575073ffffffffffffffffffffffffffffffffffffffff84163b155b15610934576040517f9996b31500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8516600482015260240161013e565b50806108a5565b80511561094b5780518082602001fd5b6040517f1425ea4200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008060006060848603121561099257600080fd5b505081359360208301359350604090920135919050565b803573ffffffffffffffffffffffffffffffffffffffff811681146109cd57600080fd5b919050565b600080600080608085870312156109e857600080fd5b843593506109f8602086016109a9565b9250610a06604086016109a9565b9150610a14606086016109a9565b905092959194509250565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b80820281158282048414176107e1576107e1610a1f565b808201808211156107e1576107e1610a1f565b600082610aae577f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b500490565b80516dffffffffffffffffffffffffffff811681146109cd57600080fd5b600080600060608486031215610ae657600080fd5b610aef84610ab3565b9250610afd60208501610ab3565b9150604084015163ffffffff81168114610b1657600080fd5b809150509250925092565b600060208284031215610b3357600080fd5b5051919050565b818103818111156107e1576107e1610a1f565b63ffffffff828116828216039080821115610b6a57610b6a610a1f565b5092915050565b60005b83811015610b8c578181015183820152602001610b74565b50506000910152565b84815283602082015273ffffffffffffffffffffffffffffffffffffffff831660408201526080606082015260008251806080840152610bdc8160a0850160208701610b71565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169190910160a00195945050505050565b600060208284031215610c2357600080fd5b815180151581146108a557600080fd5b60008251610c45818460208701610b71565b919091019291505056fea2646970667358221220a5e6159112c5a73eca08997f7dadd30c2d04c8c9a393cdeaa614c1399ffffe4a64736f6c63430008110033"
    "0x608060405234801561001057600080fd5b50600436106100675760003560e01c80637cc354f1116100505780637cc354f1146100a5578063cf62f25b146100cd578063ff53554e146100ea57600080fd5b8063054d50d41461006c57806323f0a9ab14610092575b600080fd5b61007f61007a366004610f29565b6100fd565b6040519081526020015b60405180910390f35b61007f6100a0366004610f7e565b61027d565b6100b86100b3366004610f7e565b610652565b60408051928352602083019190915201610089565b6100d5600a81565b60405163ffffffff9091168152602001610089565b6100b86100f8366004610fcb565b610aa7565b6000808411610193576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f556e697377617056324c6962726172793a20494e53554646494349454e545f4960448201527f4e5055545f414d4f554e5400000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6000831180156101a35750600082115b61022f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602860248201527f556e697377617056324c6962726172793a20494e53554646494349454e545f4c60448201527f4951554944495459000000000000000000000000000000000000000000000000606482015260840161018a565b600061023d856103e561101d565b9050600061024b848361101d565b905060008261025c876103e861101d565b6102669190611034565b90506102728183611047565b979650505050505050565b60006102a073ffffffffffffffffffffffffffffffffffffffff84168587610c14565b6000806000808773ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156102f1573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061031591906110a0565b506dffffffffffffffffffffffffffff1691506dffffffffffffffffffffffffffff1691508573ffffffffffffffffffffffffffffffffffffffff168773ffffffffffffffffffffffffffffffffffffffff16101561037957819350809250610380565b8093508192505b50506040517f054d50d4000000000000000000000000000000000000000000000000000000008152600481018890526024810183905260448101829052600090309063054d50d490606401602060405180830381865afa1580156103e8573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061040c91906110f0565b9050606461041b600a82611109565b61042b9063ffffffff168361101d565b6104359190611047565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015290915060009073ffffffffffffffffffffffffffffffffffffffff8716906370a0823190602401602060405180830381865afa1580156104a5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104c991906110f0565b90506000808773ffffffffffffffffffffffffffffffffffffffff168973ffffffffffffffffffffffffffffffffffffffff16106105095783600061050d565b6000845b604080516000815260208101918290527f022c0d9f00000000000000000000000000000000000000000000000000000000909152919350915073ffffffffffffffffffffffffffffffffffffffff8b169063022c0d9f906105779085908590309060248101611151565b600060405180830381600087803b15801561059157600080fd5b505af11580156105a5573d6000803e3d6000fd5b50506040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015285925073ffffffffffffffffffffffffffffffffffffffff8b1691506370a0823190602401602060405180830381865afa158015610615573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061063991906110f0565b61064391906111cd565b9b9a5050505050505050505050565b60008061067673ffffffffffffffffffffffffffffffffffffffff85168688610c14565b6000806000808873ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156106c7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106eb91906110a0565b506dffffffffffffffffffffffffffff1691506dffffffffffffffffffffffffffff1691508673ffffffffffffffffffffffffffffffffffffffff168873ffffffffffffffffffffffffffffffffffffffff16101561074f57819350809250610756565b8093508192505b50506040517f70a0823100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff888116600483015260009184918916906370a0823190602401602060405180830381865afa1580156107c9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107ed91906110f0565b6107f791906111cd565b6040517f054d50d4000000000000000000000000000000000000000000000000000000008152600481018290526024810185905260448101849052909550859150600090309063054d50d490606401602060405180830381865afa158015610863573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061088791906110f0565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015290915060009073ffffffffffffffffffffffffffffffffffffffff8916906370a0823190602401602060405180830381865afa1580156108f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061091b91906110f0565b90506000808973ffffffffffffffffffffffffffffffffffffffff168b73ffffffffffffffffffffffffffffffffffffffff161061095b5783600061095f565b6000845b604080516000815260208101918290527f022c0d9f00000000000000000000000000000000000000000000000000000000909152919350915073ffffffffffffffffffffffffffffffffffffffff8d169063022c0d9f906109c99085908590309060248101611151565b600060405180830381600087803b1580156109e357600080fd5b505af11580156109f7573d6000803e3d6000fd5b50506040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015285925073ffffffffffffffffffffffffffffffffffffffff8d1691506370a0823190602401602060405180830381865afa158015610a67573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a8b91906110f0565b610a9591906111cd565b97505050505050505094509492505050565b600080610acc73ffffffffffffffffffffffffffffffffffffffff8416333087610c9a565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff8416906370a0823190602401602060405180830381865afa158015610b36573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b5a91906110f0565b9150610b7d73ffffffffffffffffffffffffffffffffffffffff84163384610c14565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815233600482015273ffffffffffffffffffffffffffffffffffffffff8416906370a0823190602401602060405180830381865afa158015610be7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c0b91906110f0565b90509250929050565b60405173ffffffffffffffffffffffffffffffffffffffff838116602483015260448201839052610c9591859182169063a9059cbb906064015b604051602081830303815290604052915060e01b6020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050610ce6565b505050565b60405173ffffffffffffffffffffffffffffffffffffffff8481166024830152838116604483015260648201839052610ce09186918216906323b872dd90608401610c4e565b50505050565b6000610d0873ffffffffffffffffffffffffffffffffffffffff841683610d7c565b90508051600014158015610d2d575080806020019051810190610d2b91906111e0565b155b15610c95576040517f5274afe700000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8416600482015260240161018a565b6060610d8a83836000610d93565b90505b92915050565b606081471015610dd1576040517fcd78605900000000000000000000000000000000000000000000000000000000815230600482015260240161018a565b6000808573ffffffffffffffffffffffffffffffffffffffff168486604051610dfa9190611202565b60006040518083038185875af1925050503d8060008114610e37576040519150601f19603f3d011682016040523d82523d6000602084013e610e3c565b606091505b5091509150610e4c868383610e58565b925050505b9392505050565b606082610e6d57610e6882610ee7565b610e51565b8151158015610e91575073ffffffffffffffffffffffffffffffffffffffff84163b155b15610ee0576040517f9996b31500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8516600482015260240161018a565b5080610e51565b805115610ef75780518082602001fd5b6040517f1425ea4200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600080600060608486031215610f3e57600080fd5b505081359360208301359350604090920135919050565b803573ffffffffffffffffffffffffffffffffffffffff81168114610f7957600080fd5b919050565b60008060008060808587031215610f9457600080fd5b84359350610fa460208601610f55565b9250610fb260408601610f55565b9150610fc060608601610f55565b905092959194509250565b60008060408385031215610fde57600080fd5b82359150610c0b60208401610f55565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b8082028115828204841417610d8d57610d8d610fee565b80820180821115610d8d57610d8d610fee565b60008261107d577f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b500490565b80516dffffffffffffffffffffffffffff81168114610f7957600080fd5b6000806000606084860312156110b557600080fd5b6110be84611082565b92506110cc60208501611082565b9150604084015163ffffffff811681146110e557600080fd5b809150509250925092565b60006020828403121561110257600080fd5b5051919050565b63ffffffff82811682821603908082111561112657611126610fee565b5092915050565b60005b83811015611148578181015183820152602001611130565b50506000910152565b84815283602082015273ffffffffffffffffffffffffffffffffffffffff8316604082015260806060820152600082518060808401526111988160a085016020870161112d565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169190910160a00195945050505050565b81810381811115610d8d57610d8d610fee565b6000602082840312156111f257600080fd5b81518015158114610e5157600080fd5b6000825161121481846020870161112d565b919091019291505056fea264697066735822122097372897e36841d8634dd356f8849745adbbde7acefdfaac488bc3f0fc365e7e64736f6c63430008110033"
        .parse()
        .unwrap()
});

// adapted from: https://github.com/gnosis/evm-proxy-detection/blob/main/src/index.ts
pub static EIP_1967_LOGIC_SLOT: &str =
    "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
pub static EIP_1967_BEACON_SLOT: &str =
    "0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50";
pub static OPEN_ZEPPELIN_IMPLEMENTATION_SLOT: &str =
    "0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3";
pub static EIP_1882_LOGIC_SLOT: &str =
    "0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7";

pub static IMPLEMENTATION_SLOTS: Lazy<Vec<U256>> = Lazy::new(|| {vec![
    U256::from(EIP_1967_LOGIC_SLOT),
    U256::from(EIP_1967_BEACON_SLOT),
    U256::from(OPEN_ZEPPELIN_IMPLEMENTATION_SLOT),
    U256::from(EIP_1882_LOGIC_SLOT),
]});
