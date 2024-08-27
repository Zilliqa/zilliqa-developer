task("deployProxy", "deploy an ERC20 proxy for a ZRC2 contract")
  .addPositionalParam("zrc2Address")
  .setAction(async ({zrc2Address}) => {
    console.log(`Deploying an ERC20 proxy for the ZRC-2 at ${zrc2Address}`);
    // ZRC2 addresses frequently have bad checksums, hence toLowerCase().
    const contract = await ethers.deployContract("ZRC2ERC20Proxy",
                                                 [zrc2Address.toLowerCase()],
                                                 { gasLimit: 1_000_000 });
    await contract.waitForDeployment();
    const proxyAddress = await contract.getAddress();
    console.log(`Complete. There is now an ERC20-compliant proxy at ${proxyAddress} for the ZRC-2 contract at ${zrc2Address}`);
    console.log(`Verifying .. `);
    // verify:verify just throws unknown network errors, no matter how hard you try to disable etherscan, so ..
    await hre.run("verify:sourcify", {
      address: proxyAddress,
    });
    console.log(`... verification complete`);
  });
