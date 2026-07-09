//! Hardcoded constants for Ondo GM simulation.
//!
//! Contains program IDs, solver addresses, admin accounts, and GM token list.

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Ondo GM Program ID (mainnet production)
pub const ONDO_GM_PROGRAM_ID: &str = "XzTT4XB8m7sLD2xi6snefSasaswsKCxx5Tifjondogm";

/// Jupiter Order Engine Program ID (mainnet)
pub const JUPITER_ORDER_ENGINE_PROGRAM_ID: &str = "61DFfeTKM7trxYcPQCM78bJ794ddZprZpAwAnLiwTpYH";

/// Admin minter account (real on-chain authority for minting GM tokens)
/// This is the actual admin minter that has permission to mint GM tokens on mainnet
pub const ADMIN_MINTER: &str = "4pfyfezvwjBrsHtJpXPPKsqH9cphwSDDb7s63KzkVEqF";

/// USDC Mint (mainnet)
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

/// SPL Token Program ID
pub const SPL_TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

/// Token-2022 Program ID
pub const TOKEN_2022_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

/// Authorized Ondo GM Solver addresses
pub const AUTHORIZED_SOLVERS: [&str; 4] = [
    "AMJ81TnD4EWftmVPxppiEPsSFbmfYAvvLkUaNDXuR7JH",
    "DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds",
    "2Cq2RNFFxxPXL7teNQAji1beA2vFbBDYW5BGPBFvoN9m",
    "9BB7Tt5uE5VdRsxA5XRqrjwNaq8XtgAUQW8czA6ymUPG",
];

/// All Ondo GM token mint addresses (mainnet)
/// Format: (symbol, mint_address)
pub const GM_TOKENS: [(&str, &str); 435] = [
    ("AALon", "9wYZetvT8J2ptfsRca5gzLBGvcUug38mp9yT3xaondo"),
    ("AAOIon", "YuFZvc8JCN3a6BUAqwnbY4AnhuVEXD4V7QnBTmwondo"),
    ("AAONon", "nwPWRVFCbU3cdXWdJsuwomC5u459xPFdP2vYsmVondo"),
    ("AAPLon", "123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo"),
    ("ABBVon", "MFerpBVGKZh2jXN7cbJdXRXQTp6j6pbSnSZrfWrondo"),
    ("ABNBon", "128qNYovdGv2YqayErcJgU7gDwbNVX1VuoxbtWz8ondo"),
    ("ABTon", "129gRoHKhVg7CvPMrqVsEB4uYZo6zV4yDZX6NBg9ondo"),
    ("ACHRon", "KcCVQxG9LhFYP5o9DWFKTFgFShPPQkDEemVbiFyondo"),
    ("ACLSon", "jDoTgDRKSgVzkKdkaHZL3DiVmDM4YtYWwRG6Tgfondo"),
    ("ACMRon", "nkbH2doD7nU4CkKVwzmd6UV4d2AaGy24NHzsh6tondo"),
    ("ACNon", "12LxMMJYVSf4LoeqjFE47BQQNRciaH9E3nbDfjH4ondo"),
    ("ADBEon", "12Rh6JhfW4X5fKP16bbUdb4pcVCKDHFB48x8GG33ondo"),
    ("ADIon", "LmTMwmZLNZszn3qpjmnbhfP12U4qWDivaEBwSBSondo"),
    ("AEHRon", "ZWUSgDGQTQPGJyipzgJKcPhxgZzSBi4q6dqQbgCondo"),
    ("AGGon", "13qTjKx53y6LKGGStiKeieGbnVx3fx1bbwopKFb3ondo"),
    ("AGon", "hrZ5vs6c6v1iWyvEXjGSHs3sQuuj58VzXikNyRWondo"),
    ("AIPon", "jmnrdSzu293vKTWyEx3A2ZRVxxytJKW1wD3CLzkondo"),
    ("AIQon", "uwh6Z6c2F8WZfUSK1A8VBfA9AwJKN5T2bvQwVFLondo"),
    ("ALABon", "cskxd6aqyqJMYgLZmFYfYecWkjasRJDEtm1QVxsondo"),
    ("ALBon", "B5KufqHkskgGYwMXtL8FSHgREAkMQvE3ykhH5Kmondo"),
    ("ALOYon", "ndHvUEgrvZquSR6wZv2cG1AiBr7e7HGuWvfPULcondo"),
    ("AMATon", "7eRX747PSbVtGVx3qD5UFdkNM2BfTy86ikUiCMhondo"),
    ("AMCon", "C9xNaNujcF1a5fidWAAFReFYqhLRVbyk4yPyGqzondo"),
    ("AMDon", "14diAn5z8kjrKwSC8WLqvBqqe5YmihJhjxRxd8Z6ondo"),
    ("AMEon", "ko48myqhBuXyL9WAp6pTzHWsdKsGKSditJVoGTSondo"),
    ("AMGNon", "SS6AEWhzRrxhL2cXzKKjhFt3rCzmHHGKmFyugDTondo"),
    ("AMKRon", "nvvoP8gFyY2aZp6Cxu9Qq4MQqGrMebjtCxWrYfbondo"),
    ("AMZNon", "14Tqdo8V1FhzKsE3W2pFsZCzYPQxxupXRcqw9jv6ondo"),
    ("ANETon", "Cq6QtvHpXbJWtFaiMhUDtHy8YVZ95gcD1oZ1cohondo"),
    ("AOSLon", "b98FynyBEkdhP4Y3QUvKG36nms4oMsxEZUrCBMvondo"),
    ("APHon", "fFTZ9Jckm2X811mdqRBS4ckMz5bRAJVjH4Jwofwondo"),
    ("APLDon", "B6WqvLGXdGqpw7qgxeb5EGiRZEYo2apWpQybjYuondo"),
    ("APOon", "14VXAhoa1R74vi1ZuiQyGLJrnDMfoFBPJSCpGVz3ondo"),
    ("APPon", "14Z8rQQe2Aza33YgEUmj3g3QGNz8DXLiFPuCnsD1ondo"),
    ("ARGTon", "rki25TZmDh94spjeoyyGWjkVEYSzcVvaAbddXGuondo"),
    ("ARMon", "15SsCZqCsM9fZGhTmP4rdJTPT9WGZKazDSsgeQ8ondo"),
    ("ARQQon", "crakmaGKTuVRYqSBsFsuqio5CmpEQnpibDrVLbDondo"),
    ("ASMLon", "1eLZPRsn8bAKmoxsqDMH9Q2m2k7GMNp6RLSQGm8ondo"),
    ("ASTSon", "B6ry9goGNvVbhq7gWHzs3p6emJ1gLaMhu4By9TTondo"),
    ("ATKRon", "ZsCDHjWFyndwgbHMs4AHYwJZQMgmAn72ESQj2b5ondo"),
    ("AURon", "eGbh3V9R5ujWYwKJZAyM4Eg3sfzLhwKyr4bGbTsondo"),
    ("AVGOon", "1FWZtdWN7y38BSXGzbs8D6Shk88oL9atDNgbVz9ondo"),
    ("AXPon", "1WxT6NdK7uqpfXuKpALxL2n3f7Rq61XXeHA8UM4ondo"),
    ("AXTIon", "Z1K14ngynqmrmfRSC2dRZGs1ghmnDKiiahjaM2condo"),
    ("BABAon", "1zvb9ELBFShBCWKEk5jRTJAaPAwtVt7quEXx1X4ondo"),
    ("BACon", "Wk8gC6iTNp8dqd4ghkJ3h1giiUnyhykwHh7tYWjondo"),
    ("BAIon", "gKkrSgVjRjdQX4LFErBka1izQhoW2VHXFcCS5Vbondo"),
    ("BAon", "1YVZ4LGpq8CAhpdpm3mgy7GgPb83gJczCpxLUQ3ondo"),
    ("BBAIon", "YXE7mph6XhsgnyezkMEcTuohSuWhbLWfwx2Hh6mondo"),
    ("BEon", "bBMTGF7atoCizHMT3KCeqJzqR2gXFSUXr53AEDgondo"),
    ("BIDUon", "54CoRF2FYMZNJg9tS36xq5BUcLZ7rju1r59jGc2ondo"),
    ("BILIon", "14kLsQVmc64qZexYuR4XGop9y8BeMkd77pJUm1Rhondo"),
    ("BILon", "wtwpt5yJbButAhjpYhtg4uvUgCQN4LVgvLq2AxEondo"),
    ("BINCon", "mhZ69E1vDnAsQJXAwarLYSX5tmgeMajXBJ2rXAcondo"),
    ("BKCHon", "uyWDgDZqL6x2V86i7vwJTKPuyg2u79UYaBe5yt7ondo"),
    ("BLCRon", "g3jQMP79SxnH1KisVw3C4SBpa8gSbPAocNJruJFondo"),
    ("BLKon", "5H1VpMzRuoNtRbPTRCz35ETtEUtnkt8hJuQb9v7ondo"),
    ("BLSHon", "A9PFmw9Hu8zzxDUoU351pio1E1XWBWBfWnjT9qoondo"),
    ("BMNRon", "MYXqkDYbzr7vjXAz2BapR4AiYRXzoikGirrLoRzondo"),
    ("BNOon", "BAU83kqEqhyiexfAMQhZZE5KnGogSqh17fJc44Sondo"),
    ("BOTon", "b8UDyp3Yx19rcdaBUNegoojyUdhPpiPQ46bFrtQQQon"),
    ("BOTZon", "soLM6jRVdG1PdurSAQDz5qRtwWxXPM6EBvwrkBjondo"),
    ("BRHYon", "fznj92AnTcQ6mAFvt68JgLJS5pHag5uPmJ7LmSLondo"),
    ("BRLNon", "fcfpT8y5fpEBJjqmjKLpscZYzVjxR95ErJsb31jondo"),
    ("BRTRon", "gPwmyo4BM4qgYYCTVgA4eJmzsnYNVMUJYBecYkCondo"),
    ("BTDRon", "kBUAHgGHFthfnwarWxqYxHqVDnqqieJkXb6kvroondo"),
    ("BTGon", "cBnVXDyZgaaLZM18wAmqsUKnRUFAEJWbq6VuUoaondo"),
    ("BTGOon", "bgJWGuQxyoyFeXwzYZKBmoujVdatGFYPNFnv1a6ondo"),
    ("BWETon", "m7mWfvhyPikY3esNwTk8U1JRbcBijmzHgqiqx3xondo"),
    ("BZon", "doPqjCxi6UkANkvMz5fSuYGEo5PGppVpTZMeB5vondo"),
    ("CAMTon", "ZcS6FuJ1nAjgwJejUsxkasM6JpEDkdowVyohBCzondo"),
    ("CAPRon", "BS8zoc6pmALQnBhBDFak6eFhgGHjpebnHzsxApgondo"),
    ("CATon", "AErxJJxGbc9cZzZoZepN62BNfg5RXns8tmEc3Zpondo"),
    ("CBRSon", "c7KEKjzaTQYpTrHM2db8yq3bikPpcXrHgBV8Qcgondo"),
    ("CCJon", "fVPj4hHHVEeUrzVnad5fvxFEPGAXD5X6wkw1Xjdondo"),
    ("CEGon", "7NWHifsBnn9DimUeNnsHdEXkTZhXmJTiXxcCngBondo"),
    ("CEVAon", "nLTgFdT7x37oXMbZoZxbQ1787qSPVRLXo7JPRkLondo"),
    ("CIBRon", "BVdL3WUxtxUD4vXRWwqChJLbGxvfzZjBGPp63Wtondo"),
    ("CIENon", "cY3kDrNWP6DUZcWSQmA6Y4Nf7q5qS5kZ8zF9iLnondo"),
    ("CIFRon", "WNZBSkNBNP3Ct1pcFn6Fu4sZQFhnu48EsM9voCEondo"),
    ("CLFon", "fTuoE9pWbVK7EUpUEENBn8Vu226T7kF3YJBTRLPondo"),
    ("CLOAon", "t71FyTYHVkPAb5g48adDHmkVxXYbUuP2eq6jDZLondo"),
    ("CLOIon", "ucQ3VfWAx9pkCN4Kg84zE56FtB4FJN2kQH4ArYYondo"),
    ("CLSon", "eL1buL9zFxFhfRbjMfyPu2q9HSAJkUUnHVUgkPdondo"),
    ("CMGon", "5owVsVFSHACQuippFYdLp3qWRobp2EGcwxMmsr6ondo"),
    ("COFon", "R2uDbMtmHq5xSS5SserrovdRKdpiqnVBCd2AHLhondo"),
    ("COHRon", "BXMkru8ded26p71gJ3AMMwJmwZaYYfQjRo8vbZzondo"),
    ("COHUon", "jYMxpcgARQCdvQ15H1vvRCnBUbUEEQgSnS6SsfTondo"),
    ("COINon", "5u6KDiNJXxX4rGMfYT4BApZQC5CuDNrG6MHkwp1ondo"),
    ("Con", "PjtfUiw6Hwd8PZ94EcUw8mBSYxp7SjjzSLeNTDKondo"),
    ("COPon", "X68p9qTpEMkR1TLpXUP2ZJo8PG4Qge2Y2ZLdjA2ondo"),
    ("COPXon", "X7j77hTmjZJbepkXXBcsEapM8qNgdfihkFj6CZ5ondo"),
    ("COROon", "gNhrgh21pQozQoc7YtvhdKF7eJnSKwWa9dzHnaxondo"),
    ("CORZon", "f4ucqqnktrkdDAnwqcAAiA9Lggz6NAHJ3zFwipnondo"),
    ("COSTon", "6btaz134wjHkR8sqhAYrtSM6tavftfxnRvnyMd8ondo"),
    ("CPERon", "hpkpc1Xenv5oEpVefk3woWjZa9rxaJxaEaVVA4Fondo"),
    ("CPNGon", "NKyzy31w2J7odLb2CW3Ft4fpKXkW3LBt1pvpkVLondo"),
    ("CRCLon", "6xHEyem9hmkGtVq6XGCiQUGpPsHBaoYuYdFNZa5ondo"),
    ("CRDOon", "d4Rc6KvP3nQT8zC86Z31zM1DJCSfUD6y424cKnZondo"),
    ("CRMon", "7D7ukbcnUNYt7Et5vtsDZhAy28MKu9pkHka1Hp9ondo"),
    ("CRWDon", "cdKfoNjbXgnSuxvoajhtH3uixfZhq1YXhQsS1Rwondo"),
    ("CRWVon", "BfPGpgNyxe6rjAru1EJarjSBAcCABuMF5L32v7nondo"),
    ("CSCOon", "7DWcZE1uVc8m2mf9pV8KNov28ET7HsvHkhrhgr9ondo"),
    ("CVNAon", "FGmUDXqA3AbWfo5b3NUcsvwoUFCF4tr9ea6uercondo"),
    ("CVXon", "7tgKziACteG26VjV5xKufojKxwTgCFyTwmWUmz5ondo"),
    ("DASHon", "83P1gCFBZfGRCwJuBt9juxJKEsZwejJoG66eTZ6ondo"),
    ("DBCon", "td1aY5AvYQuwGD75qNq9aPipMexraN9mQXJwqifondo"),
    ("DELLon", "cFDP5SsUBeKrV1RkKHdaofHBSfRW8cBd7DiaPTSLAon"),
    ("DEon", "CqQyAZjB9LGFTG95eiadGTkfhd9QA12ProeKsQmondo"),
    ("DGRWon", "gnoSQSNTNZHViqVfxCcPDVxcRA29mrJL7C6JqYLondo"),
    ("DGXXon", "jwCKwGoJfx1p4K5XCwqPrq1xyJU1g26Tmf6UcDcondo"),
    ("DISon", "mJf1xT3suXtkXBCfZcE9oUUuyxkvSgqYBWiX7v1ondo"),
    ("DNNon", "12J2LD3tuLfdiVKnWZMHRMrbnXDY9rM4yqVLUa5yondo"),
    ("DRAMon", "oXeD5ZesXfJQ3mxtuZdMaccUsWrE8r1SnpYRP2Bondo"),
    ("DRSon", "gtKc3PtKfUH7vbvYLJ1HCRXCpQK1Wpgevn8e6gUondo"),
    ("DTCRon", "t29YBAB7g6xzgRJkzmc5NkQ7YRjE3NF8mhsLgppondo"),
    ("DYNFon", "g7vMfs5FrR8JjieeGC3c9sJaYPp4G3jGPfF4tkyondo"),
    ("ECHon", "BmXVAFyfpW7VuVYeWDtbFtLx7sek2mZt3BEsGgAondo"),
    ("ECOon", "mnYetf4bWKX8HihNk1XLNYj8BPPy9PdkDFPV97Zondo"),
    ("EEMon", "916SDKz7y5ZcEZC9CtnQ5Djs1Y8Yv3UAPb6bak8ondo"),
    ("EFAon", "AbvryMGnaba9oADMZk8Vp2Av6MtczsncGyfWaC4ondo"),
    ("EFVon", "uzQx2MnWr7drR5gdNXssJrFKQkLFSdw4EpfaQ5Nondo"),
    ("EMRon", "nNyVbs9Qty6wU2YcP5KFh4SUxNWTdPtL2W1bTMrondo"),
    ("ENBon", "aqEnHXRnXEQwDXEiFSEU4xHziw3Fco4b5JPkTtnondo"),
    ("ENLVon", "BncvtBGs4JqgYZwUoq3EN9q9HUFqJKTfWpvCsHCondo"),
    ("ENPHon", "Bp26APthMuM46gMFTo5KYpo7b92GN2xSCor7f9oondo"),
    ("ENTGon", "awCwGaVNbYJH2SyQJzgE3mB54gxa6SQEZSKZaHQondo"),
    ("EQIXon", "aheEdmuryJU8ymy8LjYheZH5i2BW1UMsfuWQKD2ondo"),
    ("EQTon", "bWrYATfytuRwGoDbpxy16aQbMbCZDv8DURuLCAhondo"),
    ("ETNon", "BpYiU1dBXU1fdB64jbR93wHEw3Y47QeRLZvUyLQondo"),
    ("EUHYon", "teUYhoQUgqsFp9ZwYBUHfuUHdVvbNx9N8spESGqondo"),
    ("EWJon", "C6c7VcxuUYcV5YTsky5HM4PUmfwHTwsDD5DNwwPondo"),
    ("EWYon", "C8pSaSgjkiTWixS3GM6Hxd6HKnKrgAbY9WDgfVeondo"),
    ("EWZon", "CBKcmEvVg5EgE3W5hVSPcBYWh6TFVjQwbmYod9Pondo"),
    ("EXODon", "CJRoTbu98waCCuLFfLuJ2kXawLk889fqW4UAAbwondo"),
    ("EXTRon", "js1cCZRNx8ircYiQJuhBNMnsA9owr6ZLYx6z2uNondo"),
    ("FCELon", "dYDS22uTX8CtiyixnXY9fMVGAkxbemVAjbCaWVbondo"),
    ("FCXon", "CY8ttw5rYCT6fFBJwqXofefqa7Ji9E8zfLmhRLmondo"),
    ("FFOGon", "CYAwMGyuNSDu7NpuccNwcxMNS5Bu9akxU2Jooyiondo"),
    ("FGDLon", "CYqLHM92EhmF83iNgfN4A1j2ckjsHigRvXu7xHCondo"),
    ("FIGon", "aLDdFsr3VTUQaHFK6yNvQxztvxQ8nxW4AMuSGC7ondo"),
    ("FIGRon", "ZmHxc6Gt27RJKxD2ay6UL4n9yQ7mKAq4XZQUeVhondo"),
    ("FLEXon", "iicfp8Efr4WfGAP9gXmYzdxmNFi1LV3iVudAmCnondo"),
    ("FLHYon", "CZ3FxxSto7tsjkSkqMek1C5p3RCFFmkwKqW57nbondo"),
    ("FLNCon", "bzoe1epsQLx65zmez4pWfBumYzpaFwRTvnmCjZVondo"),
    ("FLQLon", "CZ9GBn1okotqKNUUqoxk4PF2JVi59bw5GWvVo6Dondo"),
    ("FNon", "cfxyRHXjqoKN6hF3oEGu1bpEEFGcEiVXoNG4UUCondo"),
    ("Fon", "5hT2o25X9tGXipwhLckaUdgnxrZ6Y8eiUwdhpLeondo"),
    ("FORMon", "ZDnkXeN5awDioQjP691XFLdgZwDAv19g3fCr9KWondo"),
    ("FPSon", "m3XghfWMqmVE81LKLVxd1FVCKjqYAUxH8bMHGhzondo"),
    ("FTGCon", "ivBnfPTyuHDNWmMSnbavckhJK6SHZW8h77nZKsEondo"),
    ("FUTUon", "Ao5rKFRQ54W3DKSAtqfhBRPNHewwWRLNLao2JL9ondo"),
    ("FXIon", "CeFbGYXDmkyfo1TXXzzZ512mtnCCewNohu6V15vondo"),
    ("GDon", "hESwwvKsJH4p7Xib5rrM921Ng19cwcQGtxyrgSJondo"),
    ("GEMIon", "NrTdGMA3ujUvWXkwXyZKnhoByb32KTjRh5Vo47yondo"),
    ("GEon", "aTBfDuLRqYHBiG82bHA7DzwjSDTFre2dRtGH3S5ondo"),
    ("GEVon", "CgZSv89BL58ybWfWobANKEU8nV9jYfFw23G2DZEondo"),
    ("GFSon", "etnBzce6pkJq67QUv78PefkVyCEaA6YBE4hvx1Gondo"),
    ("GGOVon", "tzuC3sZnHg7spuAFhdCqivx9qtJg15qUBUpCJx1ondo"),
    ("GLDon", "hWfiw4mcxT8rnNFkk6fsCQSxoxgZ9yVhB6tyeVcondo"),
    ("GLTRon", "CgnZbDNzBfaLyJqUtd4esKLShRp7RznQuwP4uQaondo"),
    ("GLWon", "YQzNQh2YSFQ6nh91E8Ja71U6JuZDLap5jJCsELGondo"),
    ("GLXYon", "CkWmEM2J79k6AjAwyQVHXteFucAL1zQrKLxLqJHondo"),
    ("GMEon", "aznKt8v32CwYMEcTcB4bGTv8DXWStCpHrcCtyy7ondo"),
    ("GNRCon", "eqzwohR9oCR6sravF4y5HyUwyvCDbnfSYqiiFrXondo"),
    ("GOOGLon", "bbahNA5vT9WJeYft8tALrH1LXWffjwqVoUbqYa1ondo"),
    ("GRABon", "m9GcsVgdjaL3KsdtSFHimnhtsUMpTHkjtwEG4Tzondo"),
    ("GRNDon", "Gc1aT3ay7FXL3qdAW7cNSXYPDsGavy7qiACuxwxondo"),
    ("GSon", "BchJRy2snmhJZf3rQ9LJ3ePs2BGfYgfvQNo31d2ondo"),
    ("HALon", "iFcwEB2LfeYLWKgZ2vogEzC5dP7s7xbhVX81XFwondo"),
    ("HDon", "MtEXKVN3Pcggy8MPA3eJr15H6SK3RXheScqj9qtondo"),
    ("HIIon", "h73FNVBDq95fqGBy5eunHm2FVfu2jWZNkeXHDieondo"),
    ("HIMSon", "bdh3njeo19d2TBLAKTGvCWdSoArfVw8uZBAJHY4ondo"),
    ("HIMXon", "aCx5G8ewGTSzozEn8KmSsr9cvfyFWzGnr22GjFXondo"),
    ("HIVEon", "kTMQKHhnWPTvZsfiZfcdeHdG6dMgZV27wXSiC3Yondo"),
    ("HLITon", "o3pnLke4uti6hY3LTfb2wVBpHeWG7znjJHj6VXtondo"),
    ("HOODon", "BVdXGvmgi6A9oAiwWvBvP76fyTqcCNRJMM7zMN6ondo"),
    ("HPEon", "axbgKgUMscTJ34DjA69kBJuf6UYq4Pzb8B8numYondo"),
    ("HSAIon", "nagL8iWMNLZVuKFk3bUGDaHyT5ZY4bNfUzsdtGHondo"),
    ("HUBBon", "ZmiDoowvkpp1Qgx4mmY3qtsHbNV1oE12ApKCbZNondo"),
    ("HUTon", "f7iz4BQsnjw95EUyFiBKAnKgo7oBrycfzQdtmDwondo"),
    ("HYGon", "c5ug15fwZRfQhhVa6LHscFY33ebVDHcVCezYpj7ondo"),
    ("HYSon", "CsN1Tyz467bSFLPGd6MJyZhPNtwDaWZtX8ixHWyondo"),
    ("IALTon", "gfKuBLive7Q35MYgxPgNx7qx524zQJ9RiDZJFZoondo"),
    ("IAUon", "M77ZvkZ8zW5udRbuJCbuwSwavRa7bGAZYMTwru8ondo"),
    ("IBMon", "C8bZkgSxXkyT1RgxByp2teJ24hgimPLoyEYoNa9ondo"),
    ("ICHRon", "ZTABSukbFUFcuCYpMFHxN18aB4kPL2NkpZpgnXPondo"),
    ("IDEFon", "th3tot4SRq6jgEyJ568NEh42MM82RSJ3NkiWeNzondo"),
    ("IEFAon", "C9J9vZ8N79GzzxFoRkPWCkGtMKU8akg4FhUk4r9ondo"),
    ("IEFon", "D4uWxzR5StYC6sTRhVts8Eboy3pmVtHeNC62dnQondo"),
    ("IEIon", "wuzf2FDZTRbRY3ZnndMeQ38Wk9YTDGGXTA63nUyondo"),
    ("IEMGon", "cdVNL7wK8mf1UCDqM6zdrziRv4hmvqWhXeTcck2ondo"),
    ("IGEBon", "feZXF2iFspS6QKE4LSXeSESRXgrtvzbE3dZSyydondo"),
    ("IGVon", "jYxKRFuXr6PEkzPpf1wWF7DhLL4gxGJ95Pv2NGrondo"),
    ("IJHon", "cfPLN9WXD2BTkbZhRZMVXPmVSiRo44hJWRtnaC8ondo"),
    ("IJSon", "v34vtrbcjDswpFDixpVFThmUWeM1RTZwtWcp5FBondo"),
    ("INCEon", "D8KT4Jd8qiKKTfkM8ejSKCpWGR1o3GFvnQGp5ERondo"),
    ("INDAon", "DBNwt3FoYCKQWdfzxKFNZ4mzuz4Jz1iRzFf7HFzondo"),
    ("INODon", "nTUjRdtzGCy8FXHK8w1n11pHABX6Dc7L7WSpzdBondo"),
    ("INROon", "g4kT9HEg7rN4e5ZaEGHmzpkdM8qMbsZSHJojKeCondo"),
    ("INSWon", "mV5mof9x8nDirrHwT7g16MarvHbnRvz2zN2S4Cspyon"),
    ("INTCon", "cJpUMp5R7rZ6fGeLHbHhrRuJzK9mkyKDjZqNpT3ondo"),
    ("INTUon", "CozoH5HBTyyeYSQxHcWpGzd4Sq5XBaKzBzvTtN3ondo"),
    ("IONQon", "DDZQijTbaSd3Kas1r1bgCnHPayk8vTP8SfZWp5Tondo"),
    ("IRDMon", "go6DXMdM5zHTC9G16BwAYA8rKwGRhy9M5uudNdBondo"),
    ("IRENon", "13QHuepdhtJ3urNsV9i1hdL8nQoca2G7ZaLzb5FYondo"),
    ("ISRGon", "1MGRpPrkhEsCm2GCWD3rsvEU77xTTLAzfKXeFgFondo"),
    ("ITAon", "DDcAL93Urf7KrPntvKULnZoFs4Wdee1LkkJqLpjondo"),
    ("ITOTon", "CPWkMURVvcnX8hGjqCTb8i5LkzV3VSvyk7SeJi8ondo"),
    ("IVVon", "CqW2pd6dCPG9xKZfAsTovzDsMmAGKJSDBNcwM96ondo"),
    ("IWFon", "dSHPFuMMjZqt7xDYGWrexXTSkdEZAiZngqymQF2ondo"),
    ("IWMon", "dvj2kKFSyjpnyYSYppgFdAEVfgjMEoQGi9VaV23ondo"),
    ("IWNon", "DX7g7WNjDpVzNK9CG81v7wb6ZbiNzYfkdzH2Xs5ondo"),
    ("IYWon", "vr8RQPDmYQBruiYsFSV3KZyoWFEsEejxzMCdWrBondo"),
    ("JAAAon", "KZtqx9BJbpcGY7vdzhqPXM3ECKChxE5YhXaDiwRondo"),
    ("JBLon", "igWFQo1W64cQN6QUWYRXhM1UvpPuYTYtLELbDYqqqon"),
    ("JDon", "E1aUS5nyv7kaBzdQzPVJW5zfaMgoUJpKYzdnFS2ondo"),
    ("JNJon", "KUXt7LzHWSQXp5eyqMZRxWjAP6yM8BUh4LRHwiwondo"),
    ("JPMon", "E5Gczsavxcomqf6Cw1sGCKLabL1xYD2FzKxVoB4ondo"),
    ("KEELon", "kSbeWEe64qpoVb1ZSVxgRnekZ1PwGNJkLyL5gJWondo"),
    ("KEYSon", "jYcn4hHgyq1fS46YgecQY9N1gLU3sAxKA3DZVAPondo"),
    ("KLACon", "149o8ppQf9SzKCKXZ4v3dzHkwumvtQSRzSEkr29uondo"),
    ("KOon", "e6G4pfFcrdKxJuZ4YXixRFfMbpMvgXG2Mjcus71ondo"),
    ("KOPNon", "eSu547weHVErV8nax42PyJzPT8JodhBfXLDp5vyondo"),
    ("KWEBon", "DVPSYdqWPLvNa8afnEqa3B9eDfTTWpGyUZeXvdMondo"),
    ("LASRon", "Z8aFb6uQJgwFJ4KYKrT8n53aP66xqihodfAu4AKondo"),
    ("LECOon", "kmppRwWb2odH6D4JWj8Lq9WshMyyUeCYssWQFhiondo"),
    ("LEMBon", "teZfcA6zpP476eKzED1daqBWDtuwbk9e2Ejk2cpondo"),
    ("LINon", "Edik9MoFp8LAXS9HNu2gRFyihwYqDqv4ZmNmVT9ondo"),
    ("LIon", "v12TwfofSbvVqQ5N5KGG4d3J8rtEi4BjGfn2apyondo"),
    ("LITEon", "YrquZHx3f6sXsnZZAQiVsyQfvHwLmn2XTkDiZ1uondo"),
    ("LITon", "syb82jXkHWbcWgxoRqrvAcoCdsAb3y1fnCYo561ondo"),
    ("LLYon", "eGGxZwNSfuNKRqQLKaz2hc4QkA2mau7skyxPdj7ondo"),
    ("LMTon", "EoReHwUnGGekbXFHLj5rbCVKiwWqu32GrETMfw4ondo"),
    ("LOWon", "edLdFJVVR532qhcrNTJjLAmhmyV7NsctbWVokMBondo"),
    ("LPTHon", "dhEXYTmQKbYBH3wbWTMqeZZpADSRprM4jiGYbUMondo"),
    ("LRCXon", "wFJoeEYpKg9oRhyJy6BWTT3J95gmXBLvoeikDQNondo"),
    ("LSCCon", "nDetEKBEk9chztCXSYNrU5F63s2RCEQCxT7BhxDondo"),
    ("LUNRon", "DiDWPZ7vQXfpaeQ8BX68XuDYeiQLv7diDxdeUpaondo"),
    ("LWLGon", "dKGNHXGsZL4GZ4UBTCjpPbaMerqe1EdZ7aFdCxHondo"),
    ("MAon", "EsVHcyRxXFJCLMiuYLWhoDygrNe1BJGpYeZ17X7ondo"),
    ("MARAon", "ETCJUmuhs5aY62xgEVWCZ5JR8KPdeXUaJz3LuC5ondo"),
    ("MBLYon", "eCSPcjdpdKL1546PU3RM6BXkebuKn8iH4iuMcTBondo"),
    ("MCDon", "EUbJjmDt8JA222M91bVLZs211siZ2jzbFArH9N3ondo"),
    ("MEIon", "jiwgLgWJ8f6aEsM6hcSCrXLNnGpYfjmCVbqqAcwondo"),
    ("MELIon", "EWwdgGshGngcMpDV34pWZRSu5bkAuiKuKTTHKQ8ondo"),
    ("METAon", "fDxs5y12E7x7jBwCKBXGqt71uJmCWsAQ3Srkte6ondo"),
    ("MKSIon", "ZNkQTVtc4WRMQfVCC23PmwYX41577tPcvs2AiXAondo"),
    ("MPon", "XwFm5GiKPVTvPiEbQpdc6vJbFEpsUXRMf6TcSxnondo"),
    ("MRKon", "bn1fb8dwzafGePqNPrM8m8cbAKQiFqeEPuZkPySondo"),
    ("MRNAon", "14VP7DvCAdBCc5XGNZkPt6zhtPzJrWWS64Koxtxyondo"),
    ("MRVLon", "FovBwhoV5KQjZCdhoM6jgXYwXLX3F8vgAfvmLH7ondo"),
    ("MSFTon", "FRmH6iRkMr33DLG6zVLR7EM4LojBFAuq6NtFzG6ondo"),
    ("MSTRon", "FSz4ouiqXpHuGPcpacZfTzbMjScoj5FfzHkiyu2ondo"),
    ("MTSIon", "dAc8yWUrVra9v2PGsn3LT18oybsM62ysQ4ikWcpondo"),
    ("MTZon", "R3ywbVQ5t8LNmjQsn2Ngv43dSqyZscQwNag9G3Eondo"),
    ("MUon", "Fz9edBpaURPPzpKVRR1A8PENYDEgHqwx5D5th28ondo"),
    ("MXLon", "aGn43ed4kjATwbVqsAuwAT24XcG9xABCcyQsFpqondo"),
    ("MYRGon", "auLvQAhUzPuy2SQBSq2T6AofPGNkR4nZ83P8pjuondo"),
    ("NATon", "mmy8WbFRNrjoDsPGqpYmzQAVu7PfGhMCdSRLxZLondo"),
    ("NBISon", "DiRshqNDE68bWbGdLHm1GwQ76MvWQG3af6w1NdQondo"),
    ("NEEon", "t7eN6cGwRMFaZvsNW2SmVwkedmHtDdrxA4ycNE5ondo"),
    ("NEMon", "Dig28Tf1ufhCBAsjTmFkXCgcNgMqDMYj5A2rDQmondo"),
    ("NETon", "ZtAY65FCh3YB9H1wkbjRxxY5nXt9VfuTTz3Mzbuondo"),
    ("NFLXon", "g4KnPrxPLeeKkwvDmZFMtYQPM64eHeShbD55vK6ondo"),
    ("NIKLon", "V8LRV7kWjrx6Prke9oHEHNUiR122BVtyuPciTCTondo"),
    ("NIOon", "yQ37dFiGAbzrb2FRAEhGNzRy5zFfoYGWYhAepFEondo"),
    ("NKEon", "g646pcdG2Rt5DH9WZzL7VVnVDWCCMTTrnktwE74ondo"),
    ("NNEon", "bz2iUTXWkutnfwG32ziABcTzXoM91sdcgdiJJJdondo"),
    ("NOCon", "Dm6FpQ76SsbVmAZ4NvD2mjZP7cxbw1CASr4WwCiondo"),
    ("NOKon", "amE2ANm5dyG6RTkJHdtzvWcuR8ChBZCEm5Jiqwdondo"),
    ("NOWon", "G7pTVoSECz5RQWubEnTP7AC83KHUsSyoiqYR1R2ondo"),
    ("NTESon", "YeK2TdPtGLAme3Phg4pb1GBN2YxKgX5UNVyD4asondo"),
    ("NUEon", "mvAUPvwKPW4rbbTXkqCvcZEG45XCeRHSVcLVym8ondo"),
    ("NVDAon", "gEGtLTPNQ7jcg25zTetkbmF7teoDLcrfTnQfmn2ondo"),
    ("NVMIon", "nupQ2BuCfoVeCHVLRDhTjLJanaf5cxZ81KVFqs6ondo"),
    ("NVOon", "GeV7S8vjP8qdYZpdGv2Xi6e7MUMCk8NAAp2z7g5ondo"),
    ("NVTon", "Z7G1bRFYH47se4g1ppqSgtMzeJs4JjzzFPmt7iAondo"),
    ("NVTSon", "fXXYmrdSAwVmtNo1ZwrkxVep7BxTsusGzmUZJSPondo"),
    ("OIHon", "DnvbCqRuUYssmKVRBRNwkUnptHitH4ZZTt1KVuZondo"),
    ("OIIon", "gfTDvjLp8K5gNDFaLMvoTZWJJY6PmVQfdPaUU7eondo"),
    ("OKLOon", "m6oDLvJT7rY7M1TxuLWP3pWmAPg2cCWDQR1NKiEondo"),
    ("ONDSon", "7qy1j4Mechfyr6AST3djH4vk4kiEYC2cjEytXdondo"),
    ("ONon", "13qtwy5fZi9Przz14pzo9xqFSr8QHmLyUpUCvP1xondo"),
    ("ONTOon", "aHFrgfBHMGEScG8j64cN324jeoQ4EVXLmiHxtuPondo"),
    ("OPENon", "ou1uE526v7zmUYP2qCb2LJgfXAyWAtWS9SETtr8ondo"),
    ("OPRAon", "gbHFTMkuMQUy5xrgoCBdaQ2XYvNyjWAYcnRPh9Condo"),
    ("ORBXon", "t5XWftMCacS1p3xrg14ARaxgEvEM5R241kxHGqrondo"),
    ("ORCLon", "GmDADFpfwjfzZq9MfCafMDTS69MgVjtzD7Fd9a4ondo"),
    ("OSCRon", "ThwGDsXZ6iKubWuEQjmDxGwF3bUERDGbBXvcbjFondo"),
    ("OUSTon", "aV3R9NPU6TkyA6r9NPF5bmAw5XXsjUU7r2whgBqondo"),
    ("OXYon", "1GNFMryQ6c9ZpMhgNimmsbtgYM21qnBJgRAFoNiondo"),
    ("PALLon", "P7hTXnKk2d2DyqWnefp5BSroE1qjjKpKxg9SxQqondo"),
    ("PANWon", "M7hVQomhw4Q2D2op3HvBrZjHu9SryjNvD5haEZ1ondo"),
    ("PAVEon", "DsLQ18ooPjiHYuiuQ5Jz8PNCpVaKe3FhAYpvMxWondo"),
    ("PBRon", "GRciFCqJ5y2hbiD6U5mGkohY65BZTXGuGUrCqf7ondo"),
    ("PCGon", "UP5s1srLaHDc4SwJqLPa3A48x5R7ofN3hZWxWEZondo"),
    ("PDBCon", "M6agiXbNgy8Xon9ngiW4ZDPbMFcNCTMkMMkshZyondo"),
    ("PDDon", "PnjETBCLC318DRejo9cMQKAmET9PvW8AEFGWMNtondo"),
    ("PENGon", "dWFwjcUKdc7bPH9GEebpJVmEmUjvQTgEWGVR9WYondo"),
    ("PEPon", "gud6b3fYekjhMG5F818BALwbg2vt4JKoow59Md9ondo"),
    ("PFEon", "Gwh9fPsX1qWATXy63vNaJnAFfwebWQtZaVmPko6ondo"),
    ("PGon", "GZ8v4NdSG7CTRZqHMgNsTPRULeVi8CpdWd9wZY8ondo"),
    ("PINSon", "sxyg1VTSzy5zYANUK7hntNtmFAWoXGJq95AcHuVondo"),
    ("PLon", "aq2zXUHqx7Zk6HSJH2GYsNajQJZj9f3dV7gAzfuondo"),
    ("PLTRon", "HfsnTS5qtdStwec9DfBrunRqnAMYMMz1kjv9Hu9ondo"),
    ("PLUGon", "TnfswqdE1jAJ8sfnf5J7kSVLEH1cfpAYZ8MWmKfondo"),
    ("POWIon", "ZifkbVBh94FSETjAfoLw587nxmGsYtXayAAUQgzondo"),
    ("POWLon", "fRFzaZfGSXPf2r4oBgBBXpisRovMaJZjv1aCQBsondo"),
    ("PPLTon", "DwRtkbsaQMGAS3oMeEGYh6M5vH4X9WECsQgqHjAondo"),
    ("PRIMon", "kcc5QzXDCQ61qQ5Nbpi2RppnRSzhG1XQNXkjXwoondo"),
    ("PSQon", "qKtU9A7ij34XmtxaSzYfxCpkgAZzzFsqnUb2kW2ondo"),
    ("PURRon", "rsiKbHCdsvmExvDfkYWypAXFsqKz6V8XuoxbkHtondo"),
    ("PWRon", "f1yQz2fo7S24NqrsfaWDkmQ8xoa8yU72c9rEEBdondo"),
    ("PYPLon", "hM7B3UQTTR81mS27SxDDPzBbjejmo8fnpFjzgv9ondo"),
    ("QBTSon", "hqJXutLF6f7DxStrWCrnZDfXzbNTZmvi3KheVi6ondo"),
    ("QCOMon", "hrmX7MV5hifoaBVjnrdpz698yABxrbBNAcWtWo9ondo"),
    ("QLTAon", "fkznXN9GALK7f9zVr2RwRHWCPhwoima4zB3JbbNondo"),
    ("QQQon", "HrYNm6jTQ71LoFphjVKBTdAE4uja7WsmLG8VxB8ondo"),
    ("QTUMon", "iJAAwDNzJHbgKm5pksL3kXHc3zZewYm37dsZNCPondo"),
    ("QUBTon", "E4YowrHx5wm4RtSjfuvTqtNH3Wf7NEj5tYZGD9Bondo"),
    ("QYLDon", "ueEYw3Djy9GVu9mrP6jum8qNpxshgcy7gMfmntWondo"),
    ("RDDTon", "HXFrTf9v9NdjGUTnx4sojR3Cf92hoBsQFUxKTN7ondo"),
    ("RDWon", "E6KSaqjvqe2HiUpbEweRxLK4RimQddigm95H9Jaondo"),
    ("REGNon", "E86mX2yb3HLbJM6gRtZQ6dCYmLh6MSDZadu9SCPondo"),
    ("REMXon", "tiitb2Z1HtpB2DpVr6V7tdCFS3jmTinLeuGj9EVondo"),
    ("RGTIon", "dwEPNKQab3iwRmjGvZPXhAmws1W5NsQGwuXwi8oondo"),
    ("RIOTon", "i6f3DvZBuLpnGSqS8x6WPeStJ7jNe5KewD6afD5ondo"),
    ("RIVNon", "AXRsYFt7TXNQ3DcY6BkvRgPV6VsYMURyDtaeudjondo"),
    ("RKLBon", "E9VQY3VnrpVSekFByzRmfeK1kxgM3UiKCoVVbdUondo"),
    ("RMBSon", "jjnSEAsi8UbCez7x9XCbWntLWRHBdc2tWSdC3uoondo"),
    ("ROKon", "e83tWWrVsVk1hRGNz5BCwNr9TMBNWixmoUhWgYcondo"),
    ("RTXon", "12BvLZtzjdssAycxPeBQUjukhmgQpULAvy6SroYdondo"),
    ("SAPon", "bjbrNi96mXAzgvxSuGJ2SRJ5U4N8agbG7wUAKAjondo"),
    ("SATAon", "vZVGEJfSM1hS4XdFVAZL2Fr1cbPzJty9vWyax68ondo"),
    ("SBETon", "iLDu2jjp2i3Uqc2Vm7K7GLiUj3hR4Un49MtD7c4ondo"),
    ("SBUXon", "iPFqjcZQTNMNXA4kbShbMhfAVD8yr8Uq9UtXMV6ondo"),
    ("SCCOon", "EANjzFjj3nPXHdzN5CE3Z8LLVn69Ce77FE8X4cvondo"),
    ("SCHWon", "cnc6M1zXLdrGR5LAQVcaJDfgezMiVWNtGQsVy1Kondo"),
    ("SEDGon", "EAwP9LGNjTkQ2YeKE6CGKqBYtrJ6APFvRe7KCMmondo"),
    ("SGOVon", "HjrN6ChZK2QRL6hMXayjGPLFvxhgjwKEy135VRjondo"),
    ("SHLDon", "siVse6kjZb9ihaXHaqoG3mhHyTPEnNCkvSDTheoondo"),
    ("SHOPon", "ivdDracs2s7jCP698dJXKSEQdVrNj9hasJL1Uq1ondo"),
    ("SHYon", "EEy57xbaLcUrN1HXj2vz8VWxeWFK1eZQZo4aWbrondo"),
    ("SILon", "uiSLmtLdqxtbQq5gkwYBvBrZpnSNXZn8h6sjLsDondo"),
    ("SKHYon", "Huyb2fyDDjSuDKCRWsN9ci2rmcgPo6NFiLbx9ZDondo"),
    ("SLBon", "i7ZS13SF6BCKbzvLujp2UqLNMgM1XVnZ7A7wC6tondo"),
    ("SLVon", "iy11ytbSGcUnrjE6Lfv78TFqxKyUESfku1FugS9ondo"),
    ("SMCIon", "jLca79XzcewRuBZyaJxVxuKpUHcEix1X4CP1RP9ondo"),
    ("SMRon", "bGy2covWNf5qyzoNdV1pWXuLmFi6Dq927o7JXzWondo"),
    ("SNAPon", "a2cXfonVgQ6cKB4Lm8YZsPry39VZSA562bwmRSiondo"),
    ("SNDKon", "EJmUVvDqAdfH5zEohkdS4234bi3c6iunqEMobjmondo"),
    ("SNOWon", "JmFLCBwoNvcXy6B2VqABg6m784ubkXpaEx3p7S5ondo"),
    ("SOFIon", "mqL8yXQpeSvc7NgrAtLLPtRvUiWyLoG5RWLv16iondo"),
    ("SOon", "aKzjn2ZdWySSGPSSDTY2HUpcSCmemSahTXihrpyondo"),
    ("SOUNon", "vE2qArmjto6VfeMngyGAnzp2ipLYeXsxiARDnnXondo"),
    ("SOXLon", "isRSJECP9yFPv9YejzGUdjzAGHbF2x5DpVeDqAiondo"),
    ("SOXQon", "io3eLhnjT1a94JpzAUMWKqwMYHZRwvtGXjkkXsPondo"),
    ("SOXSon", "ivnSAcjCqEtWYTKFbqYe8YoqRZqCBfT4BGP5G1nondo"),
    ("SOXXon", "EN5pHc1LccUSojxb7kkyQi7v7iJN5RpDq6qz3DHondo"),
    ("SPCXon", "wzAyQTorWyoVXuJKj2x8EqKEGJpS13z6EWE9z5Aondo"),
    ("SPGIon", "JrTYw7A9jihX5TwpRStYviEbsYf2X2VJpZ13719ondo"),
    ("SPOTon", "jzCvs2Pk8tDcfsFRqnEMjurgaQW4iQfEkandUR8ondo"),
    ("SPYon", "k18WJUULWheRkSpSquYGdNNmtuE2Vbw1hpuUi92ondo"),
    ("SQQQon", "D1tu7Fnm3cCpKyyPXrqm5GXShPqMj7a2SEjjq9fondo"),
    ("STLDon", "n7DwzSkv1SBkcA9qj8LU9sZ9sRn72Z6spU2w2b9ondo"),
    ("STMon", "bM2VSRfbYPt29YRD9F2wTCSCSQaHtNCuz1znNDCondo"),
    ("STNGon", "mFyszXnJf8BFR8H4o33pCZS1T36BH9LjtG3gTpdondo"),
    ("STXon", "EXtprP1wzrNo2bByrU9JyzqEg2hQMSCVJakeHHYondo"),
    ("SWKSon", "iJtKb1CWnWdgJhs7HgSZvLmSJABGGMc97QeuG7tondo"),
    ("SYMon", "nP42LxpSZkUfnBUxiFsHxL5GKYWRZ1VxqGkMTNwondo"),
    ("TASKon", "nQysX1ZsRJ8yTJg8smZTZ91rWcVBabDRqdUEKZHondo"),
    ("TCOMon", "9PMjLqd8zPdKkJUXarnit5t7tPL3cCscwHzy7ATondo"),
    ("TELon", "ZjYCwYeG85TbV5oXkCkvWQTNPh2PgTQ8X4nxpbyondo"),
    ("TENon", "micfqeFfvD9iDKKzuqRHXerFxG8K5VfY8CgrcQoondo"),
    ("TERon", "ahvtJqt6pkzjnYTMaCKrvjPQszSKyWraiXKvuWKondo"),
    ("TIPon", "k6BPp2Xmf2TYgrZiUyWfUoZBKeqaDbvPoAVgSx2ondo"),
    ("TLNon", "RTb54gpqAx6RpLAHRGnqQ3ciQ845CHqhg21ZzEJondo"),
    ("TLTon", "KaSLSWByKy6b9FrCYXPEJoHmLpuFZtTCJk1F1Z9ondo"),
    ("TMon", "kbmF7ERJWMaaDswMprrH9gHSLya5D2RMBNgKqg3ondo"),
    ("TMOon", "T699bgtXQw4CJ59rQ4VzLsupVQUzoL5RmuhHnKrondo"),
    ("TMUSon", "pDY4GPJfZcNETPG7myXeafQfgJqqVkn81bMYDyfondo"),
    ("TNKon", "mPAqB3y5N7fWmEh1BoVtrLhZKBkQe7LjBCrYUNbondo"),
    ("Ton", "WKMZummev5UcXz5nNKQZvTD6QjNSM2X58uwmDReondo"),
    ("TQQQon", "14W1itEkV7k1W819mLSknFTaMmkCtPokbF2tRkPUondo"),
    ("TSEMon", "cRx9VtwwPTZbVk1DjbMyKzrMWn7nJA22UpMyzFYondo"),
    ("TSLAon", "KeGv7bsfR4MheC1CkmnAVceoApjrkvBhHYjWb67ondo"),
    ("TSMon", "keybg184d4vyXeQdFqs4o99YsMg7xBthxTJ6Ky3ondo"),
    ("TTMIon", "kWmjV2XdK5tbV6kZrM8grS6EGFmuH5i5HFW3YyLondo"),
    ("TTon", "erp2t2My8UoFgyRt39EmnnSiDUwUM5aNKw5piBKondo"),
    ("TXNon", "81xLFvCzFaUM3KDxSHC75pXu3RPCeSeCbmGBY8aondo"),
    ("UAMYon", "nbwNoPaFYNY2c3u4iK6U59ySC2ehFrpjdpfbyLDondo"),
    ("UBERon", "KJNeFW3kk3ycPjXpC6cbuyckjeYHacc2ekhtAi5ondo"),
    ("UCTTon", "jCPs1JpVKAwevND3jzeDGAUBBFkJ5TUtiu2SxLbondo"),
    ("UECon", "EYo8D3cLdF1CDeGms5M5VHyU52HJYinkMZ1cqvYondo"),
    ("UMCon", "ieocA48cBX3oiVECgosMGxG649wnf7R8EkVrA5fondo"),
    ("UNGon", "Es2ipHL7qXBcLmZ4N7LP9PHBHaWaTMTAkxDwGGjondo"),
    ("UNHon", "kPBGL8vAwKN3UGmr9cjkM2dU79SC3nzTC9yu7F8ondo"),
    ("UNPon", "EvsME8gdnEwPLbTnhrGVDwrY35zBuB8hEGCq59Hondo"),
    ("URAon", "EvzskrQ3vUUkiMGG1DzfSDyG6H2WCMy3v9G8fzzondo"),
    ("URNMon", "hieZTEZNBU67bMGULK9hWCB9h5jBPKdpRWiXpwkondo"),
    ("USARon", "aA1dRckexLmQyppFoWmjKDFjrNFUsZeGzZ7L5xpondo"),
    ("USFRon", "o6U1Sm6Vd7EofMyCrL28mrp2QLzgYGgjveHiEQ5ondo"),
    ("USOon", "rpydAzWdCy85HEmoQkH5PVxYtDYQWjmLxgHHadxondo"),
    ("UUUUon", "ey16y4Bk92zmPSvbRznuv3RioAXbVreBkQxrKGDondo"),
    ("VCXon", "esgtAV7yKf7Ei3Q92VmXcEGkoqY2UqCHzZvCWhgondo"),
    ("VDEon", "hKVpWfYwP1VJ9BcBTPRcovcSpEkvnaN8eXwFoCMondo"),
    ("VFSon", "F3V1fKLKv7H8aNdt9TC6GQ3X4LayEfGHsPi8Umaondo"),
    ("VICRon", "f9nfUo4SdhCGfHmm81m3ArgDsatwo2jLjEcgCcYondo"),
    ("VNQon", "F3dMJ9H137YUNc9cpN3gBWDSq4MSRbTFtojH65Uondo"),
    ("Von", "kxEW4oJL75K37VeXaZF1ynbHQATQwhECQKN1374ondo"),
    ("VPGon", "dYF78b65HS62V3pku2uYFyektYzAhx9YACv4hWfondo"),
    ("VRSNon", "ja4bMvHL3Hw9Ey33VGWyDeXvrHvQWyBnK4GSmCUondo"),
    ("VRTon", "MkN2TZSYTFBdMRLf9EVcfhstTwnazH8knd9hpepondo"),
    ("VRTXon", "FL7QzUq58pvkDxkftJm7RqRWgqYEFZwXuvAMsUnondo"),
    ("VSHon", "jbzBdFNddeEiJXGcVH4DE2qUyYfTtyY2vaHJDEZondo"),
    ("VSTon", "h6MW8GFpfzxFa1JNn6hZNnBF3t4fj9SHAXKy6LXondo"),
    ("VTIon", "jCCU4GwukjNxAXJowG2S4KCrr5g6YyUB61WHYvGondo"),
    ("VTVon", "KuiYLPVq65qixD9TgvxBC576C4gG6vVTCdbh2zFondo"),
    ("VZon", "igu1coP6n3GPaWmbd8J9Z7UAyLpV254uQFFNfydondo"),
    ("WCCon", "m3m2HAANsAf2Y3BkdBixDgtrrFHnZDp4NqVh9obondo"),
    ("WDCon", "FLqH2jB2DZPJP5nnVFAakRKaNTcDZtq71Pnpp6Aondo"),
    ("WFCon", "L6ZE5qCpVVSqLePz64CrwkgyWoPF9M7tB8BeFH4ondo"),
    ("WLKon", "mrNSd1y72F7Dx2Uip4vidtsJKKd8iJatTKGX6Pvondo"),
    ("WMBon", "bvjmEwQBqbMr6rnx5a74boBz6nmA1DNThujPnNAondo"),
    ("WMon", "FPvKvWzSzDZqgYmSZUetrkpUXSwo2VtpR4BynVYondo"),
    ("WMTon", "LZddqAqKqJW9oMZSjTxCUmbmzBRQtv9gMkD9hZ3ondo"),
    ("WOLFon", "Zfb5PTVfGa8AV6VxrTQJuP8CjMXFPMVkVVNpcAWondo"),
    ("WSon", "ZpJpMhWKCr4m9ZzxApJJwDc5cHiHp2hG1RZdJyvondo"),
    ("WULFon", "exYfSJt6Fgfhfnp3bAD4roYy97hLF9npjYaLyEXondo"),
    ("WYFIon", "jtnRMv1U3bJHQCsi47E6Lf8Nzkaqsisef7SkHBgondo"),
    ("XOMon", "qCYD74QnXzd9pzv6pGHQKJVwoibL6sNcPQDnpDiondo"),
    ("XYLDon", "wCr7YFeYDWyYSebsoMY75g8c9pguGeVB3rT6kYjondo"),
    ("XYZon", "BWxe2FVciUbwrCUZQPUKiREBh5LmVa5AiUqNLAkondo"),
    ("YEARon", "wxPFbh4dVrTWPGHHbVVeTHH7GK2uQwnTm5C8X3Fondo"),
];

/// Get the Ondo GM program ID
pub fn ondo_gm_program_id() -> Pubkey {
    Pubkey::from_str(ONDO_GM_PROGRAM_ID).expect("Invalid Ondo GM program ID")
}

/// Get the Jupiter Order Engine program ID
pub fn jupiter_order_engine_program_id() -> Pubkey {
    Pubkey::from_str(JUPITER_ORDER_ENGINE_PROGRAM_ID).expect("Invalid Jupiter program ID")
}

/// Get the admin minter account (real on-chain authority)
pub fn admin_minter() -> Pubkey {
    Pubkey::from_str(ADMIN_MINTER).expect("Invalid admin minter")
}

/// Get the USDC mint
pub fn usdc_mint() -> Pubkey {
    Pubkey::from_str(USDC_MINT).expect("Invalid USDC mint")
}

/// Get the SPL Token program ID
pub fn spl_token_program_id() -> Pubkey {
    Pubkey::from_str(SPL_TOKEN_PROGRAM_ID).expect("Invalid SPL Token program ID")
}

/// Get the Token-2022 program ID
pub fn token_2022_program_id() -> Pubkey {
    Pubkey::from_str(TOKEN_2022_PROGRAM_ID).expect("Invalid Token-2022 program ID")
}

/// Check if a pubkey is an authorized Ondo GM solver
pub fn is_authorized_solver(pubkey: &Pubkey) -> bool {
    let pubkey_str = pubkey.to_string();
    AUTHORIZED_SOLVERS.contains(&pubkey_str.as_str())
}

/// Check if a pubkey is an Ondo GM token mint
pub fn is_gm_token(pubkey: &Pubkey) -> bool {
    let pubkey_str = pubkey.to_string();
    GM_TOKENS.iter().any(|(_, addr)| *addr == pubkey_str)
}

/// Get the symbol for a GM token mint address
pub fn get_gm_token_symbol(pubkey: &Pubkey) -> Option<&'static str> {
    let pubkey_str = pubkey.to_string();
    GM_TOKENS
        .iter()
        .find(|(_, addr)| *addr == pubkey_str)
        .map(|(symbol, _)| *symbol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_authorized_solver() {
        let solver = Pubkey::from_str("DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds").unwrap();
        assert!(is_authorized_solver(&solver));

        let random = Pubkey::new_unique();
        assert!(!is_authorized_solver(&random));
    }

    #[test]
    fn test_is_gm_token() {
        let aapl = Pubkey::from_str("123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo").unwrap();
        assert!(is_gm_token(&aapl));
        assert_eq!(get_gm_token_symbol(&aapl), Some("AAPLon"));

        let random = Pubkey::new_unique();
        assert!(!is_gm_token(&random));
    }
}
