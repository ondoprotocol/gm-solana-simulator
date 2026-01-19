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
pub const AUTHORIZED_SOLVERS: [&str; 3] = [
    "DSqMPMsMAbEJVNuPKv1ZFdzt6YvJaDPDddfeW7ajtqds",
    "2Cq2RNFFxxPXL7teNQAji1beA2vFbBDYW5BGPBFvoN9m",
    "9BB7Tt5uE5VdRsxA5XRqrjwNaq8XtgAUQW8czA6ymUPG",
];

/// All Ondo GM token mint addresses (mainnet)
/// Format: (symbol, mint_address)
pub const GM_TOKENS: [(&str, &str); 202] = [
    ("AALon", "9wYZetvT8J2ptfsRca5gzLBGvcUug38mp9yT3xaondo"),
    ("AAPLon", "123mYEnRLM2LLYsJW3K6oyYh8uP1fngj732iG638ondo"),
    ("ABBVon", "MFerpBVGKZh2jXN7cbJdXRXQTp6j6pbSnSZrfWrondo"),
    ("ABNBon", "128qNYovdGv2YqayErcJgU7gDwbNVX1VuoxbtWz8ondo"),
    ("ABTon", "129gRoHKhVg7CvPMrqVsEB4uYZo6zV4yDZX6NBg9ondo"),
    ("ACHRon", "KcCVQxG9LhFYP5o9DWFKTFgFShPPQkDEemVbiFyondo"),
    ("ACNon", "12LxMMJYVSf4LoeqjFE47BQQNRciaH9E3nbDfjH4ondo"),
    ("ADBEon", "12Rh6JhfW4X5fKP16bbUdb4pcVCKDHFB48x8GG33ondo"),
    ("ADIon", "LmTMwmZLNZszn3qpjmnbhfP12U4qWDivaEBwSBSondo"),
    ("AGGon", "13qTjKx53y6LKGGStiKeieGbnVx3fx1bbwopKFb3ondo"),
    ("AMATon", "7eRX747PSbVtGVx3qD5UFdkNM2BfTy86ikUiCMhondo"),
    ("AMCon", "C9xNaNujcF1a5fidWAAFReFYqhLRVbyk4yPyGqzondo"),
    ("AMDon", "14diAn5z8kjrKwSC8WLqvBqqe5YmihJhjxRxd8Z6ondo"),
    ("AMGNon", "SS6AEWhzRrxhL2cXzKKjhFt3rCzmHHGKmFyugDTondo"),
    ("AMZNon", "14Tqdo8V1FhzKsE3W2pFsZCzYPQxxupXRcqw9jv6ondo"),
    ("ANETon", "Cq6QtvHpXbJWtFaiMhUDtHy8YVZ95gcD1oZ1cohondo"),
    ("APOon", "14VXAhoa1R74vi1ZuiQyGLJrnDMfoFBPJSCpGVz3ondo"),
    ("APPon", "14Z8rQQe2Aza33YgEUmj3g3QGNz8DXLiFPuCnsD1ondo"),
    ("ARMon", "15SsCZqCsM9fZGhTmP4rdJTPT9WGZKazDSsgeQ8ondo"),
    ("ASMLon", "1eLZPRsn8bAKmoxsqDMH9Q2m2k7GMNp6RLSQGm8ondo"),
    ("AVGOon", "1FWZtdWN7y38BSXGzbs8D6Shk88oL9atDNgbVz9ondo"),
    ("AXPon", "1WxT6NdK7uqpfXuKpALxL2n3f7Rq61XXeHA8UM4ondo"),
    ("BABAon", "1zvb9ELBFShBCWKEk5jRTJAaPAwtVt7quEXx1X4ondo"),
    ("BACon", "Wk8gC6iTNp8dqd4ghkJ3h1giiUnyhykwHh7tYWjondo"),
    ("BAon", "1YVZ4LGpq8CAhpdpm3mgy7GgPb83gJczCpxLUQ3ondo"),
    ("BBAIon", "YXE7mph6XhsgnyezkMEcTuohSuWhbLWfwx2Hh6mondo"),
    ("BIDUon", "54CoRF2FYMZNJg9tS36xq5BUcLZ7rju1r59jGc2ondo"),
    ("BILIon", "14kLsQVmc64qZexYuR4XGop9y8BeMkd77pJUm1Rhondo"),
    ("BINCon", "mhZ69E1vDnAsQJXAwarLYSX5tmgeMajXBJ2rXAcondo"),
    ("BLKon", "5H1VpMzRuoNtRbPTRCz35ETtEUtnkt8hJuQb9v7ondo"),
    ("BLSHon", "A9PFmw9Hu8zzxDUoU351pio1E1XWBWBfWnjT9qoondo"),
    ("BMNRon", "MYXqkDYbzr7vjXAz2BapR4AiYRXzoikGirrLoRzondo"),
    ("BTGon", "cBnVXDyZgaaLZM18wAmqsUKnRUFAEJWbq6VuUoaondo"),
    ("BTGOon", "bgJWGuQxyoyFeXwzYZKBmoujVdatGFYPNFnv1a6ondo"),
    ("BZon", "doPqjCxi6UkANkvMz5fSuYGEo5PGppVpTZMeB5vondo"),
    ("CATon", "AErxJJxGbc9cZzZoZepN62BNfg5RXns8tmEc3Zpondo"),
    ("CEGon", "7NWHifsBnn9DimUeNnsHdEXkTZhXmJTiXxcCngBondo"),
    ("CIFRon", "WNZBSkNBNP3Ct1pcFn6Fu4sZQFhnu48EsM9voCEondo"),
    ("CLOAon", "t71FyTYHVkPAb5g48adDHmkVxXYbUuP2eq6jDZLondo"),
    ("CLOIon", "ucQ3VfWAx9pkCN4Kg84zE56FtB4FJN2kQH4ArYYondo"),
    ("CMGon", "5owVsVFSHACQuippFYdLp3qWRobp2EGcwxMmsr6ondo"),
    ("COFon", "R2uDbMtmHq5xSS5SserrovdRKdpiqnVBCd2AHLhondo"),
    ("COINon", "5u6KDiNJXxX4rGMfYT4BApZQC5CuDNrG6MHkwp1ondo"),
    ("Con", "PjtfUiw6Hwd8PZ94EcUw8mBSYxp7SjjzSLeNTDKondo"),
    ("COPon", "X68p9qTpEMkR1TLpXUP2ZJo8PG4Qge2Y2ZLdjA2ondo"),
    ("COPXon", "X7j77hTmjZJbepkXXBcsEapM8qNgdfihkFj6CZ5ondo"),
    ("COSTon", "6btaz134wjHkR8sqhAYrtSM6tavftfxnRvnyMd8ondo"),
    ("CPNGon", "NKyzy31w2J7odLb2CW3Ft4fpKXkW3LBt1pvpkVLondo"),
    ("CRCLon", "6xHEyem9hmkGtVq6XGCiQUGpPsHBaoYuYdFNZa5ondo"),
    ("CRMon", "7D7ukbcnUNYt7Et5vtsDZhAy28MKu9pkHka1Hp9ondo"),
    ("CRWDon", "cdKfoNjbXgnSuxvoajhtH3uixfZhq1YXhQsS1Rwondo"),
    ("CSCOon", "7DWcZE1uVc8m2mf9pV8KNov28ET7HsvHkhrhgr9ondo"),
    ("CVNAon", "FGmUDXqA3AbWfo5b3NUcsvwoUFCF4tr9ea6uercondo"),
    ("CVXon", "7tgKziACteG26VjV5xKufojKxwTgCFyTwmWUmz5ondo"),
    ("DASHon", "83P1gCFBZfGRCwJuBt9juxJKEsZwejJoG66eTZ6ondo"),
    ("DBCon", "td1aY5AvYQuwGD75qNq9aPipMexraN9mQXJwqifondo"),
    ("DEon", "CqQyAZjB9LGFTG95eiadGTkfhd9QA12ProeKsQmondo"),
    ("DGRWon", "gnoSQSNTNZHViqVfxCcPDVxcRA29mrJL7C6JqYLondo"),
    ("DISon", "mJf1xT3suXtkXBCfZcE9oUUuyxkvSgqYBWiX7v1ondo"),
    ("DNNon", "12J2LD3tuLfdiVKnWZMHRMrbnXDY9rM4yqVLUa5yondo"),
    ("EEMon", "916SDKz7y5ZcEZC9CtnQ5Djs1Y8Yv3UAPb6bak8ondo"),
    ("EFAon", "AbvryMGnaba9oADMZk8Vp2Av6MtczsncGyfWaC4ondo"),
    ("EQIXon", "aheEdmuryJU8ymy8LjYheZH5i2BW1UMsfuWQKD2ondo"),
    ("FIGon", "aLDdFsr3VTUQaHFK6yNvQxztvxQ8nxW4AMuSGC7ondo"),
    ("FIGRon", "ZmHxc6Gt27RJKxD2ay6UL4n9yQ7mKAq4XZQUeVhondo"),
    ("Fon", "5hT2o25X9tGXipwhLckaUdgnxrZ6Y8eiUwdhpLeondo"),
    ("FTGCon", "ivBnfPTyuHDNWmMSnbavckhJK6SHZW8h77nZKsEondo"),
    ("FUTUon", "Ao5rKFRQ54W3DKSAtqfhBRPNHewwWRLNLao2JL9ondo"),
    ("GEMIon", "NrTdGMA3ujUvWXkwXyZKnhoByb32KTjRh5Vo47yondo"),
    ("GEon", "aTBfDuLRqYHBiG82bHA7DzwjSDTFre2dRtGH3S5ondo"),
    ("GLDon", "hWfiw4mcxT8rnNFkk6fsCQSxoxgZ9yVhB6tyeVcondo"),
    ("GMEon", "aznKt8v32CwYMEcTcB4bGTv8DXWStCpHrcCtyy7ondo"),
    ("GOOGLon", "bbahNA5vT9WJeYft8tALrH1LXWffjwqVoUbqYa1ondo"),
    ("GRABon", "m9GcsVgdjaL3KsdtSFHimnhtsUMpTHkjtwEG4Tzondo"),
    ("GRNDon", "Gc1aT3ay7FXL3qdAW7cNSXYPDsGavy7qiACuxwxondo"),
    ("GSon", "BchJRy2snmhJZf3rQ9LJ3ePs2BGfYgfvQNo31d2ondo"),
    ("HDon", "MtEXKVN3Pcggy8MPA3eJr15H6SK3RXheScqj9qtondo"),
    ("HIMSon", "bdh3njeo19d2TBLAKTGvCWdSoArfVw8uZBAJHY4ondo"),
    ("HOODon", "BVdXGvmgi6A9oAiwWvBvP76fyTqcCNRJMM7zMN6ondo"),
    ("HYGon", "c5ug15fwZRfQhhVa6LHscFY33ebVDHcVCezYpj7ondo"),
    ("IAUon", "M77ZvkZ8zW5udRbuJCbuwSwavRa7bGAZYMTwru8ondo"),
    ("IBMon", "C8bZkgSxXkyT1RgxByp2teJ24hgimPLoyEYoNa9ondo"),
    ("IEFAon", "C9J9vZ8N79GzzxFoRkPWCkGtMKU8akg4FhUk4r9ondo"),
    ("IEMGon", "cdVNL7wK8mf1UCDqM6zdrziRv4hmvqWhXeTcck2ondo"),
    ("IJHon", "cfPLN9WXD2BTkbZhRZMVXPmVSiRo44hJWRtnaC8ondo"),
    ("INTCon", "cJpUMp5R7rZ6fGeLHbHhrRuJzK9mkyKDjZqNpT3ondo"),
    ("INTUon", "CozoH5HBTyyeYSQxHcWpGzd4Sq5XBaKzBzvTtN3ondo"),
    ("IRENon", "13QHuepdhtJ3urNsV9i1hdL8nQoca2G7ZaLzb5FYondo"),
    ("ISRGon", "1MGRpPrkhEsCm2GCWD3rsvEU77xTTLAzfKXeFgFondo"),
    ("ITOTon", "CPWkMURVvcnX8hGjqCTb8i5LkzV3VSvyk7SeJi8ondo"),
    ("IVVon", "CqW2pd6dCPG9xKZfAsTovzDsMmAGKJSDBNcwM96ondo"),
    ("IWFon", "dSHPFuMMjZqt7xDYGWrexXTSkdEZAiZngqymQF2ondo"),
    ("IWMon", "dvj2kKFSyjpnyYSYppgFdAEVfgjMEoQGi9VaV23ondo"),
    ("IWNon", "DX7g7WNjDpVzNK9CG81v7wb6ZbiNzYfkdzH2Xs5ondo"),
    ("JAAAon", "KZtqx9BJbpcGY7vdzhqPXM3ECKChxE5YhXaDiwRondo"),
    ("JDon", "E1aUS5nyv7kaBzdQzPVJW5zfaMgoUJpKYzdnFS2ondo"),
    ("JNJon", "KUXt7LzHWSQXp5eyqMZRxWjAP6yM8BUh4LRHwiwondo"),
    ("JPMon", "E5Gczsavxcomqf6Cw1sGCKLabL1xYD2FzKxVoB4ondo"),
    ("KLACon", "149o8ppQf9SzKCKXZ4v3dzHkwumvtQSRzSEkr29uondo"),
    ("KOon", "e6G4pfFcrdKxJuZ4YXixRFfMbpMvgXG2Mjcus71ondo"),
    ("LINon", "Edik9MoFp8LAXS9HNu2gRFyihwYqDqv4ZmNmVT9ondo"),
    ("LIon", "v12TwfofSbvVqQ5N5KGG4d3J8rtEi4BjGfn2apyondo"),
    ("LLYon", "eGGxZwNSfuNKRqQLKaz2hc4QkA2mau7skyxPdj7ondo"),
    ("LMTon", "EoReHwUnGGekbXFHLj5rbCVKiwWqu32GrETMfw4ondo"),
    ("LOWon", "edLdFJVVR532qhcrNTJjLAmhmyV7NsctbWVokMBondo"),
    ("LRCXon", "wFJoeEYpKg9oRhyJy6BWTT3J95gmXBLvoeikDQNondo"),
    ("MAon", "EsVHcyRxXFJCLMiuYLWhoDygrNe1BJGpYeZ17X7ondo"),
    ("MARAon", "ETCJUmuhs5aY62xgEVWCZ5JR8KPdeXUaJz3LuC5ondo"),
    ("MCDon", "EUbJjmDt8JA222M91bVLZs211siZ2jzbFArH9N3ondo"),
    ("MELIon", "EWwdgGshGngcMpDV34pWZRSu5bkAuiKuKTTHKQ8ondo"),
    ("METAon", "fDxs5y12E7x7jBwCKBXGqt71uJmCWsAQ3Srkte6ondo"),
    ("MPon", "XwFm5GiKPVTvPiEbQpdc6vJbFEpsUXRMf6TcSxnondo"),
    ("MRKon", "bn1fb8dwzafGePqNPrM8m8cbAKQiFqeEPuZkPySondo"),
    ("MRNAon", "14VP7DvCAdBCc5XGNZkPt6zhtPzJrWWS64Koxtxyondo"),
    ("MRVLon", "FovBwhoV5KQjZCdhoM6jgXYwXLX3F8vgAfvmLH7ondo"),
    ("MSFTon", "FRmH6iRkMr33DLG6zVLR7EM4LojBFAuq6NtFzG6ondo"),
    ("MSTRon", "FSz4ouiqXpHuGPcpacZfTzbMjScoj5FfzHkiyu2ondo"),
    ("MTZon", "R3ywbVQ5t8LNmjQsn2Ngv43dSqyZscQwNag9G3Eondo"),
    ("MUon", "Fz9edBpaURPPzpKVRR1A8PENYDEgHqwx5D5th28ondo"),
    ("NEEon", "t7eN6cGwRMFaZvsNW2SmVwkedmHtDdrxA4ycNE5ondo"),
    ("NFLXon", "g4KnPrxPLeeKkwvDmZFMtYQPM64eHeShbD55vK6ondo"),
    ("NIKLon", "V8LRV7kWjrx6Prke9oHEHNUiR122BVtyuPciTCTondo"),
    ("NIOon", "yQ37dFiGAbzrb2FRAEhGNzRy5zFfoYGWYhAepFEondo"),
    ("NKEon", "g646pcdG2Rt5DH9WZzL7VVnVDWCCMTTrnktwE74ondo"),
    ("NOWon", "G7pTVoSECz5RQWubEnTP7AC83KHUsSyoiqYR1R2ondo"),
    ("NTESon", "YeK2TdPtGLAme3Phg4pb1GBN2YxKgX5UNVyD4asondo"),
    ("NVDAon", "gEGtLTPNQ7jcg25zTetkbmF7teoDLcrfTnQfmn2ondo"),
    ("NVOon", "GeV7S8vjP8qdYZpdGv2Xi6e7MUMCk8NAAp2z7g5ondo"),
    ("OKLOon", "m6oDLvJT7rY7M1TxuLWP3pWmAPg2cCWDQR1NKiEondo"),
    ("ONDSon", "7qy1j4Mechfyr6AST3djH4vk4kiEYC2cjEytXdondo"),
    ("ONon", "13qtwy5fZi9Przz14pzo9xqFSr8QHmLyUpUCvP1xondo"),
    ("OPENon", "ou1uE526v7zmUYP2qCb2LJgfXAyWAtWS9SETtr8ondo"),
    ("OPRAon", "gbHFTMkuMQUy5xrgoCBdaQ2XYvNyjWAYcnRPh9Condo"),
    ("ORCLon", "GmDADFpfwjfzZq9MfCafMDTS69MgVjtzD7Fd9a4ondo"),
    ("OSCRon", "ThwGDsXZ6iKubWuEQjmDxGwF3bUERDGbBXvcbjFondo"),
    ("OXYon", "1GNFMryQ6c9ZpMhgNimmsbtgYM21qnBJgRAFoNiondo"),
    ("PALLon", "P7hTXnKk2d2DyqWnefp5BSroE1qjjKpKxg9SxQqondo"),
    ("PANWon", "M7hVQomhw4Q2D2op3HvBrZjHu9SryjNvD5haEZ1ondo"),
    ("PBRon", "GRciFCqJ5y2hbiD6U5mGkohY65BZTXGuGUrCqf7ondo"),
    ("PCGon", "UP5s1srLaHDc4SwJqLPa3A48x5R7ofN3hZWxWEZondo"),
    ("PDBCon", "M6agiXbNgy8Xon9ngiW4ZDPbMFcNCTMkMMkshZyondo"),
    ("PDDon", "PnjETBCLC318DRejo9cMQKAmET9PvW8AEFGWMNtondo"),
    ("PEPon", "gud6b3fYekjhMG5F818BALwbg2vt4JKoow59Md9ondo"),
    ("PFEon", "Gwh9fPsX1qWATXy63vNaJnAFfwebWQtZaVmPko6ondo"),
    ("PGon", "GZ8v4NdSG7CTRZqHMgNsTPRULeVi8CpdWd9wZY8ondo"),
    ("PINSon", "sxyg1VTSzy5zYANUK7hntNtmFAWoXGJq95AcHuVondo"),
    ("PLTRon", "HfsnTS5qtdStwec9DfBrunRqnAMYMMz1kjv9Hu9ondo"),
    ("PLUGon", "TnfswqdE1jAJ8sfnf5J7kSVLEH1cfpAYZ8MWmKfondo"),
    ("PSQon", "qKtU9A7ij34XmtxaSzYfxCpkgAZzzFsqnUb2kW2ondo"),
    ("PYPLon", "hM7B3UQTTR81mS27SxDDPzBbjejmo8fnpFjzgv9ondo"),
    ("QBTSon", "hqJXutLF6f7DxStrWCrnZDfXzbNTZmvi3KheVi6ondo"),
    ("QCOMon", "hrmX7MV5hifoaBVjnrdpz698yABxrbBNAcWtWo9ondo"),
    ("QQQon", "HrYNm6jTQ71LoFphjVKBTdAE4uja7WsmLG8VxB8ondo"),
    ("RDDTon", "HXFrTf9v9NdjGUTnx4sojR3Cf92hoBsQFUxKTN7ondo"),
    ("REMXon", "tiitb2Z1HtpB2DpVr6V7tdCFS3jmTinLeuGj9EVondo"),
    ("RGTIon", "dwEPNKQab3iwRmjGvZPXhAmws1W5NsQGwuXwi8oondo"),
    ("RIOTon", "i6f3DvZBuLpnGSqS8x6WPeStJ7jNe5KewD6afD5ondo"),
    ("RIVNon", "AXRsYFt7TXNQ3DcY6BkvRgPV6VsYMURyDtaeudjondo"),
    ("RTXon", "12BvLZtzjdssAycxPeBQUjukhmgQpULAvy6SroYdondo"),
    ("SBETon", "iLDu2jjp2i3Uqc2Vm7K7GLiUj3hR4Un49MtD7c4ondo"),
    ("SBUXon", "iPFqjcZQTNMNXA4kbShbMhfAVD8yr8Uq9UtXMV6ondo"),
    ("SCHWon", "cnc6M1zXLdrGR5LAQVcaJDfgezMiVWNtGQsVy1Kondo"),
    ("SGOVon", "HjrN6ChZK2QRL6hMXayjGPLFvxhgjwKEy135VRjondo"),
    ("SHOPon", "ivdDracs2s7jCP698dJXKSEQdVrNj9hasJL1Uq1ondo"),
    ("SLVon", "iy11ytbSGcUnrjE6Lfv78TFqxKyUESfku1FugS9ondo"),
    ("SMCIon", "jLca79XzcewRuBZyaJxVxuKpUHcEix1X4CP1RP9ondo"),
    ("SNAPon", "a2cXfonVgQ6cKB4Lm8YZsPry39VZSA562bwmRSiondo"),
    ("SNOWon", "JmFLCBwoNvcXy6B2VqABg6m784ubkXpaEx3p7S5ondo"),
    ("SOFIon", "mqL8yXQpeSvc7NgrAtLLPtRvUiWyLoG5RWLv16iondo"),
    ("SOon", "aKzjn2ZdWySSGPSSDTY2HUpcSCmemSahTXihrpyondo"),
    ("SOUNon", "vE2qArmjto6VfeMngyGAnzp2ipLYeXsxiARDnnXondo"),
    ("SPGIon", "JrTYw7A9jihX5TwpRStYviEbsYf2X2VJpZ13719ondo"),
    ("SPOTon", "jzCvs2Pk8tDcfsFRqnEMjurgaQW4iQfEkandUR8ondo"),
    ("SPYon", "k18WJUULWheRkSpSquYGdNNmtuE2Vbw1hpuUi92ondo"),
    ("SQQQon", "D1tu7Fnm3cCpKyyPXrqm5GXShPqMj7a2SEjjq9fondo"),
    ("TCOMon", "9PMjLqd8zPdKkJUXarnit5t7tPL3cCscwHzy7ATondo"),
    ("TIPon", "k6BPp2Xmf2TYgrZiUyWfUoZBKeqaDbvPoAVgSx2ondo"),
    ("TLNon", "RTb54gpqAx6RpLAHRGnqQ3ciQ845CHqhg21ZzEJondo"),
    ("TLTon", "KaSLSWByKy6b9FrCYXPEJoHmLpuFZtTCJk1F1Z9ondo"),
    ("TMon", "kbmF7ERJWMaaDswMprrH9gHSLya5D2RMBNgKqg3ondo"),
    ("TMOon", "T699bgtXQw4CJ59rQ4VzLsupVQUzoL5RmuhHnKrondo"),
    ("TMUSon", "pDY4GPJfZcNETPG7myXeafQfgJqqVkn81bMYDyfondo"),
    ("Ton", "WKMZummev5UcXz5nNKQZvTD6QjNSM2X58uwmDReondo"),
    ("TQQQon", "14W1itEkV7k1W819mLSknFTaMmkCtPokbF2tRkPUondo"),
    ("TSLAon", "KeGv7bsfR4MheC1CkmnAVceoApjrkvBhHYjWb67ondo"),
    ("TSMon", "keybg184d4vyXeQdFqs4o99YsMg7xBthxTJ6Ky3ondo"),
    ("TXNon", "81xLFvCzFaUM3KDxSHC75pXu3RPCeSeCbmGBY8aondo"),
    ("UBERon", "KJNeFW3kk3ycPjXpC6cbuyckjeYHacc2ekhtAi5ondo"),
    ("UNHon", "kPBGL8vAwKN3UGmr9cjkM2dU79SC3nzTC9yu7F8ondo"),
    ("USFRon", "o6U1Sm6Vd7EofMyCrL28mrp2QLzgYGgjveHiEQ5ondo"),
    ("USOon", "rpydAzWdCy85HEmoQkH5PVxYtDYQWjmLxgHHadxondo"),
    ("Von", "kxEW4oJL75K37VeXaZF1ynbHQATQwhECQKN1374ondo"),
    ("VRTon", "MkN2TZSYTFBdMRLf9EVcfhstTwnazH8knd9hpepondo"),
    ("VSTon", "h6MW8GFpfzxFa1JNn6hZNnBF3t4fj9SHAXKy6LXondo"),
    ("VTIon", "jCCU4GwukjNxAXJowG2S4KCrr5g6YyUB61WHYvGondo"),
    ("VTVon", "KuiYLPVq65qixD9TgvxBC576C4gG6vVTCdbh2zFondo"),
    ("VZon", "igu1coP6n3GPaWmbd8J9Z7UAyLpV254uQFFNfydondo"),
    ("WFCon", "L6ZE5qCpVVSqLePz64CrwkgyWoPF9M7tB8BeFH4ondo"),
    ("WMTon", "LZddqAqKqJW9oMZSjTxCUmbmzBRQtv9gMkD9hZ3ondo"),
    ("WULFon", "exYfSJt6Fgfhfnp3bAD4roYy97hLF9npjYaLyEXondo"),
    ("XOMon", "qCYD74QnXzd9pzv6pGHQKJVwoibL6sNcPQDnpDiondo"),
    ("XYZon", "BWxe2FVciUbwrCUZQPUKiREBh5LmVa5AiUqNLAkondo"),
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
