set -e

attrib=1
color=35
owner_id=nhadinh.testnet

rm -rf ./neardev ./ft-contract/neardev
printf %b "\n\033[$attrib;${color}m|| => REMOVE OLD ACCOUNT ./neardev and ./ft-contract/neardev\033[m\n"

cd "ft-contract"

printf %b "\n\033[$attrib;${color}m|| => STARTING BUILDING FT-CONTRACT...\033[m\n"
sh ./build.sh

wait
printf %b "\n\033[$attrib;${color}m|| => STARTING DEPLOY FT-CONTRACT...\033[m\n"
near dev-deploy --wasmFile ./res/ft-contract.wasm

wait
ft_contract=$(<neardev/dev-account)

cd ".."

cp ft-contract/res/*.wasm ./out/
printf %b "\n\033[$attrib;${color}m|| => STARTING BUILDING STAKING CONTRACT...\033[m\n"
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p out
cp target/wasm32-unknown-unknown/release/*.wasm out/contract.wasm

wait
printf %b "\n\033[$attrib;${color}m|| => STARTING DEPLOY STAKING CONTRACT...\033[m\n"
near dev-deploy out/contract.wasm

wait
contract_id=$(<neardev/dev-account)

printf %b "\033[$attrib;${color}m|| => Initial FT Contract with default meta data\033[m\n"
near call $ft_contract new_default_meta '{"owner_id": "'$contract_id'", "total_supply": "1000000000000000"}' --accountId $contract_id

printf %b "\033[$attrib;${color}m|| => Initial Staking Contract by default\033[m\n"
near call $contract_id new_default '{"ft_contract_id": "'$ft_contract'"}' --accountId $owner_id

printf %b "\033[$attrib;${color}m|| ----------------------------------------------------\033[m\n"
printf %b "\033[$attrib;${color}m|| => Staking contract id: '$contract_id'\033[m\n"
printf %b "\033[$attrib;${color}m|| => FT contract id: '$ft_contract'\033[m\n"
printf %b "\033[$attrib;${color}m|| => Owner community account id: '$owner_id'\033[m\n"
printf %b "\033[$attrib;${color}m|| ----------------------------------------------------\033[m\n"
