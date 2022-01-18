# rust_starter
## Running the project locally

If you want to test this project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy

# install front end dependencies
npm install

# run front end 
npm start
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

The front end app will be available as `http://localhost:8080`
