package com.zilliqa.laksaj.jsonrpc;

import com.zilliqa.laksaj.blockchain.*;
import com.zilliqa.laksaj.account.Wallet;
import com.zilliqa.laksaj.transaction.TransactionFactory;
import com.zilliqa.laksaj.exception.ZilliqaAPIException;
import com.zilliqa.laksaj.transaction.PendingStatus;
import com.zilliqa.laksaj.transaction.Transaction;
import com.zilliqa.laksaj.transaction.TransactionStatus;
import com.zilliqa.laksaj.transaction.TxPending;
import okhttp3.OkHttpClient;
import org.junit.Assert;
import org.junit.Test;
import org.junit.BeforeClass;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.TimeUnit;


public class ProviderTest {
  public static final String MAINNET_ENDPOINT_URL = "https://api.zilliqa.com";
  public static final String TESTNET_ENDPOINT_URL = "https://dev-api.zilliqa.com";
  // Use with "mitmweb --mode reverse:https://dev-api.zilliqa.com -p 8082"
  //public static final String TESTNET_ENDPOINT_URL = "http://localhost:8082";
  public static String transactionId;
  
  /** We are about to query a bunch of txn statuses; Zilliqa only holds transaction status for
   * "reasonably recent" transactions, so we create a txn here to query later.
   */
  @BeforeClass
  public static  void setUp() throws Exception  {
    transactionId = submitTransactionToTestNet();
    System.out.println(String.format("Build txnId %s", transactionId));
  }

  public static String submitTransactionToTestNet() throws Exception {
    Wallet wallet = new Wallet();
    String privateKey = "d451dbafe179ca68aa0184875be26718a8d81de217e4cfa70ae1fc08341c1c6e";
        // Populate the wallet with an account
        String address = wallet.addByPrivateKey(privateKey);
        wallet.addByPrivateKey(privateKey);

        HttpProvider provider = new HttpProvider(TESTNET_ENDPOINT_URL);
        //get balance
        HttpProvider.BalanceResult balanceResult = provider.getBalance(address).getResult();
        Integer nonce = Integer.parseInt(balanceResult.getNonce());

        System.out.println("Generating testnet transaction .. ");
        //construct non-contract transaction
        Transaction transaction = Transaction.builder()
                                  .version(String.valueOf(pack(333, 1)))
                                  .toAddr("zil16jrfrs8vfdtc74yzhyy83je4s4c5sqrcasjlc4")
                                  .senderPubKey("02dd3c5fc0cf19109bffc89d9c6dbe13dce0c49a4553b8bde2c3b41977293a03e1")
                                  .amount("10000010")
                                  .gasPrice("2000000000")
                                  .gasLimit("50")
                                  .code("")
                                  .data("")
                                  .provider(new HttpProvider(TESTNET_ENDPOINT_URL))
                                  .nonce(Integer.toString(nonce+1))
                                  .build();

        //sign transaction
        transaction = wallet.sign(transaction);

        //broadcast transaction
        HttpProvider.CreateTxResult result = TransactionFactory.createTransaction(transaction);
        transaction.confirm(result.getTranID(), 100, 3);
        return result.getTranID();
    }

    public static int pack(int a, int b) {
        return (a << 16) + b;
    }

    @Test
    public void getNetWorkId() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        String networkId = client.getNetworkId().getResult();
        Assert.assertEquals("1", networkId);
    }

    @Test
    public void getDSBlockListing() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        BlockList blockList = client.getDSBlockListing(1).getResult();
        System.out.println(blockList);
        Assert.assertNotNull(blockList);
    }

    @Test
    public void getTxBlockListing() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        BlockList blockList = client.getTxBlockListing(1).getResult();
        System.out.println(blockList);
        Assert.assertNotNull(blockList);
    }

    @Test
    public void getBlockchainInfo() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        BlockchainInfo blockchainInfo = client.getBlockchainInfo().getResult();
        System.out.println(blockchainInfo);
        Assert.assertNotNull(blockchainInfo);
    }


    @Test
    public void getDsBlock() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        DsBlock dsBlock = client.getDsBlock("1").getResult();
        System.out.println(dsBlock);
        Assert.assertNotNull(dsBlock);
        Assert.assertTrue(dsBlock.getHeader().getDifficulty() == 3);
    }


    @Test
    public void getNumDSBlocks() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        String result = client.getNumDSBlocks().getResult();
        System.out.println(result);
        Assert.assertNotNull(result);
    }


    @Test
    public void getTxBlock() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        TxBlock txBlock = client.getTxBlock("123").getResult();
        System.out.println(txBlock);
    }

    @Test
    public void getLatestDsBlock() throws IOException, ZilliqaAPIException {
        OkHttpClient client = new OkHttpClient().newBuilder()
                .writeTimeout(1, TimeUnit.MINUTES)
                .readTimeout(1, TimeUnit.MINUTES)
                .connectTimeout(1, TimeUnit.MINUTES)
                .build();
        HttpProvider provider = new HttpProvider(MAINNET_ENDPOINT_URL, client);
        DsBlock dsBlock = provider.getLatestDsBlock().getResult();
        Assert.assertNotNull(dsBlock);
        System.out.println(dsBlock);
    }

    @Test
    public void getLatestTxBlock() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        TxBlock txBlock = client.getLatestTxBlock().getResult();
        System.out.println(txBlock);
        Assert.assertNotNull(txBlock);
    }

    @Test
    public void getBalance() throws IOException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        HttpProvider.BalanceResult balance = client.getBalance("AE9C49CAF0D0BC9D7C769391E8BDA2028F824CF3F".toLowerCase()).getResult();
        Assert.assertNotNull(balance.getBalance());
    }

    @Test
    public void getBalanceWithRetry() throws IOException, InterruptedException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        HttpProvider.BalanceResult balance = client.getBalanceWithRetry("AE9C49CAF0D0BC9D7C769391E8BDA2028F824CF3F".toLowerCase()).getResult();
        Assert.assertNotNull(balance.getBalance());
    }

    @Test
    public void getBalance32() throws Exception {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        HttpProvider.BalanceResult balance = client.getBalance32("zil1z6rpmumewzrmdz44wu9hgvdwrs5xgptlzd6kec").getResult();
        Assert.assertNotNull(balance);
    }

    @Test
    public void getSmartContractCode() throws IOException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        try {
            String code = client.getSmartContractCode("8cb841ef4f1f61d44271e167557e160434bd6d63").getResult().getCode();
            System.out.println(code);
        } catch (ZilliqaAPIException e) {
            System.out.println(e.getMessage());
        }
    }

    @Test
    public void getMinimumGasPrice() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        String price = client.getMinimumGasPrice().getResult();
        System.out.println(price);

    }

    @Test
    public void getTransactionStatus() throws IOException {
        System.out.println("Running getTransaction3()");      
        HttpProvider client = new HttpProvider(TESTNET_ENDPOINT_URL);
        TransactionStatus transaction = client.getTransactionStatus(this.transactionId).getResult();
        System.out.println(transaction);
    }

    @Test
    public void getTransactionStatusWithRetry() throws IOException, InterruptedException {
        System.out.println("Running getTransaction2()");
        HttpProvider client = new HttpProvider(TESTNET_ENDPOINT_URL);
        TransactionStatus transaction = client.getTransactionStatusWithRetry(transactionId).getResult();
        System.out.println(transaction);
    }

    @Test
    public void getTransaction() throws IOException, Exception {
        System.out.println("Running getTransaction()");
        HttpProvider client = new HttpProvider(TESTNET_ENDPOINT_URL);
        String txnId = submitTransactionToTestNet();
        TransactionData transaction = client.getTransaction(transactionId).getResult();
        System.out.println(transaction);
    }

    @Test
    public void getTransactionsForTxBlock() throws IOException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        Rep<List<List<String>>> rep = client.getTransactionsForTxBlock("120951");
        System.out.println(rep);
    }

    @Test
    public void getTransaction32() throws Exception {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        TransactionData transaction = client.getTransaction32("ce918e4c77ed40f3a23588bd3c380458b43be168935d468e2e6f680724e71474").getResult();
        System.out.println(transaction);
    }

    @Test
    public void getRecentTransactions() throws IOException, ZilliqaAPIException {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        TransactionList transactionList = client.getRecentTransactions().getResult();
        System.out.println(transactionList);
    }

    @Test
    public void getSmartContractState() throws IOException, ZilliqaAPIException {
      HttpProvider client = new HttpProvider("https://api.zilliqa.com");
      // This is a copy of the Hello World contract deployed by <richard@zilliqa.com>
      String stateList = client.getSmartContractState("d9719a5682ce84eab30aee162fafcbb4acaf6757");
        System.out.println(stateList);
    }

    @Test
    public void getSmartContractSubState() throws IOException {
        HttpProvider client = new HttpProvider("https://api.zilliqa.com");
        List<Object> param = new ArrayList<>();
        // This is a multisig proxy for the staking contract on mainnet.
        param.add("6c75f531b5d5b528ddd56b1ed87d0359e80bf796");
        param.add("owners");
        param.add(new ArrayList<>());
        String state = client.getSmartContractSubState(param);
        System.out.println(state);
    }

    @Test
    public void parseError() {
        HttpProvider client = new HttpProvider(MAINNET_ENDPOINT_URL);
        HttpProvider.Pair pair = client.parseError("{\"error\":{\"code\":-8,\"data\":null,\"message\":\"Address size not appropriate\"},\"id\":\"1\",\"jsonrpc\":\"2.0\"}\n");
        Assert.assertEquals("Address size not appropriate", pair.getMessage());
    }
}
