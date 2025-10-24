use getrandom;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use crate::err::MlDsaError;
use crate::ntt::{mod_q, ntt, ntt_add, ntt_inverse, ntt_multiply};
use crate::params::{D, ETA, K, L, N, Q};
use hex;

// https://github.com/post-quantum-cryptography/KAT/blob/main/MLDSA/kat_MLDSA_44_hedged_raw.rsp
pub fn key_gen () -> Result<[u8; 1312], MlDsaError>{
    let mut xi = [0u8; 32];
    let _ = getrandom::fill(&mut xi)
        .map_err(|_| MlDsaError::KegGenRandomSeedError);
    {
        let xi = hex::decode("D71361C000F9A7BC99DFB425BCB6BB27C32C36AB444FF3708B2D93B4E66D5B5B").unwrap();
        let _pk_expected = "B845FA2881407A59183071629B08223128116014FB58FF6BB4C8C9FE19CF5B0BD77B16648A344FFE486BC3E3CB5FAB9ABC4CC2F1C34901692BEC5D290D815A6CDF7E9710A3388247A7E0371615507A572C9835E6737BF30B92A796FFF3A10A730C7B550924EB1FB6D56195F02DE6D3746F9F330BEBE990C90C4D676AD415F4268D2D6B548A8BCDF27FDD467E6749C0F87B71E85C2797694772BBA88D4F1AC06C7C0E91786472CD76353708D6BBC5C28E9DB891C3940E879052D30C8FD10965CBB8EE1BD79B060D37FB839098552AABDD3A57AB1C6A82B0911D1CF148654AA5613B07014B21E4A1182B4A5501671D112F5975FB0C8A2AC45D575DC42F48977FF37FFF421DB27C45E79F8A9472007023DF0B64205CD9F57C02CE9D1F61F2AE24F7139F5641984EE8DF783B9EA43E997C6E19D09E062AFCA56E4F76AAAB8F66600FC78F6AB4F6785690D185816EE35A939458B60324EEFC60E64B11FA0D20317ACB6CB29AA03C775F151672952689FA4F8F838329CB9E6DC9945B6C7ADE4E7B663578F87D3935F2A1522097AD5042A0D990A628510B6103CB242CD8A3AFC1A5ADA52331F4DF461BC1DA51D1D224094E7ABED3D87D98F0D817084780EE80370F397631ECB75D4264B6B5E2E66C0586B5FB743516399165837A0FDFF7C6134F033BFA69C1B2416965C6E578592F40E258CB6DFB29FB8E0F54355B6E24A65F67ABAE3193D007115CC0B9FF94CB911A93B1A76C0E7662F5E2B20139E0159ED929CB932D4895F89A02E55C59DF2DBB8F6E5DD7D5B1F3CEC37B4A9166B381C5440E23E67368CDE0A29D59AA05A3C9BE24A4DC8DD75BE30E82BC635D36AAC66DE880C6701A987D7E05F0F2FF287828BEC30595089D8AB9AA390ED719CAA6E576CDBBE9B184A322E5E2DABB69C23CC696D54FC32FF57001B6B64E2A837F3062D85AEB50B3510F7EDFC34DF38E083D4D9B94FFAB0DE15D73D9AF30B9F31CC4F41C9C24F2D618B2A7C3C4BDFB745D52D3EB54589C8BDA8AC05DAD14EC744505575A0988EEC651C1715439FDFB29923380A43C1A66A86C982A841F11820A6A0E1E2F2FFF5108ECAE51A6AABC9B949226D228FF84C4E5E5D63114D80359C4931E612DCED1838B7D066AC9182CECFA223A21A4C8E155AEFA780373BCC15098AEE40C033AF22F8E7C67A0D2526DA7475E830308C04AED9D32BCCC72E719EE70A8D13F09AC11E26EA237D5CC8F98B5AE0E54F933BD0507942ED900D056FD32F8E6E81777912FD482746029B71CCE3BA69B8FC2D03EB441027C387BC2F95031A0AE7052215EB24B9EA8FB0A961B0F80BFA80D0D6257C1C22B508C5D31B97FCDFE1D1766E8A9C8771932DD598ADB7E717743F45FC571F21E4A516249F81D747F15329790F0F70A0B8E461A4EDF50504AF03F30DDF8A8818E38761E1681D6DDEF0B1DD326B2EC228CE48570F285B49D29D7C2EF37866D5446DF82B8E43B34CB248962A21A9A3946159740F8AEE8E6A16A4EB2B42D143FE2612E05EF4B5E646D813248444556A2A8BF92CE10BADECB6B8A40B080DD42D53346FEFCC4B9B40B1E4998991EC753C95AA2F2A506F311E710B0F1D36C1DCA6644EE6D1D4AE9CEA5666EF4B3E888DBDBB95A77ECFE1E8B477DE7CB07639D682D53020EC14EA6C7DD7E715389D10938429FAB8A068A1466A4CD891359F8074E0F5A142ADD731B87878D985E4FA6ECB3B73D298553418273E9503AA84092C080E5F2902F90F5C59944D24CA0271D11D0D6734606D039550A37FCA2B735850E63F540F2F06B79144B5C4ED2C700BB51C33D265B3D037389C99EFD597642D829DB1EB58643CFCD07F4DEC60B8F727D97BD7C4B59BDA1";
        let pk = key_gen_internal(&xi.try_into().unwrap());
        assert_eq!(hex::encode_upper(pk), _pk_expected);
    }
    {
        let xi = hex::decode("AB611F971C44D1B755D289E0FCFEE70F0EB5D9FDFB1BC31CA894A75794235AF8").unwrap();
        let _pk_expected = "D712599A161ECD99EF5B7A04313D5507D612565F03AA9695ED7C2DF1CFA18056264432056C416013F5AFE21ADB7F4A5CECA1EE694E580301B47EB634E18C25BC0C964470BEBF10AFDF7587BAE7FC40D45F94519A661A7CD04F7AA0D95AAEE797027D8CBFAEE682ADD859106CCA978F5311A3B53A1699691C1E9E8BF9D2118B23BB1B33C5E9A02F968C800560A865FB826289FB34C936C825D1D0A7DB566B9F30B8E42E7A9AE8F325E51C197CEA36DDB79DF537D1F4102DD44C01B59401770BCCF4D5E8329121863929C91DFCA9C00F862ED7DBF3CF7558F6150B2B05D32F04C1E6E775350248DBFC0AEE58681B5B177CA7BA17ED6C7B4BA7302DC0F349564DB0D83C7F7D1AFE502E27DC465704AA9B7AA50D8F5A14CCC5701FAE3505D3AE11EAD8142AF9EE7991B3B216686126627C60E1769EC805000A6D94076905630D385B7CDA7195B54522A81E8855F3D6DAC03E686CADF72A9142363770283915034ECDF9047F80A8F9594273C7637B514BAF9ACE79A7B5D28F8B0EFD63A1F8B17301F499FD37E448F5E99F118FC7B2A4BFD7DADA350456CE5CC72C5646C57229ECD91082A85EFA259E2AAFF761F0A101A2269C8B88D94F7A8E4F87E7B9F977396E71BCD2B2BF89AB8B829A345D37FE5240CE7216C64EFB01BA8A13700D4BFC9856EACE22FB763BEC7D1E02F0F49B45F587BA6B3F571BE7AEBEA88557492C6C766D7AE298C7902F346DEDA494EEAC39C8767199252C9CE18825385596956F1F41B2BB534F761FA0C89917077B23F8056AA7405F8AE4271B11DF284D4FF50AFCDBFDCBE880131B60E3534091A759531FBE6E14D6057BDB458449AE4BCEC26580877331359E794E9D75BF454122264CCD13F9349C8250F0BB586B5BB482EB74524FD12F0F61EDD28411D072FCEC556BBC5569AFCEF1A62C26078088A77ADA5976E6A2A99E98C898E5789DE786B45708271957D88594DB7FBABE23DDBF1ABB12E715BD576483EAD259BA6553E95AE65AEDB55751532B98F32376BADDD2532E5801A255FA6DFC3FCCD5FC5110EE231450DDA25C4E09B89B577B8589C7179B06D62ABA652BD6DAD5159E8D7B84A5879F075EBD31ECCE03D2F1E508FB229665B4232B905638176AFCF232D4C19AF38A812C8D2AF2776CD444E94D46816EE68E7B5CD5AC1622518682EEE88E0F64358181EF35A8EC71E00E70F1222F494615A4FDF7C769CADB0E13B2F15B5EDA38809A5960D25090C1513636DE242BBD9A79F77CB45C81557D6F1D18991B37BECD4CAD9707B1F88920B69A55CD52CDAC2318287D4B13BC9CBA6884F9A38054493BDF91B29C2B7A71E2DD8ECFCDD0259BFFF402251CA2C95042AF917EBB669ACE5B354DC393A10F66847D546C47EDA695AD3A985834292E68A024AB6045ADF36DD2A58993A292B24837F61711CB59EA8DEB103F63F971E385CE0490A0709C890CAF57ED86991ACA729DBEFEB023D4356B7144485B7AF45559C3FF722EC8FCD8BEFBAF41119A24C9B686A79C38356F3FC55720F03AC89B3187CECC0C1247B9F8DDD82B8B415BAA6BC7A25E2529C41CC9765C768DC6A555D6114707879E810BC4C7472A07E8B90B0F41B8B4CABD5326B190572B664127E2FB476D88DB32D42D50BCD7F637AB2B7D13E0A7D03DFA30B0B81F7D2917C1CA5260BF71230F7342A5BB9F9261D966CDEE85F768A9B8B509F094C60E0C08BAC278827E84E6590141FF8C10E9B51FBF6E7724030EEB8B59731E47FA3D130407F833455386B66CBC453FF4BACC0467D747A1037EA57B3DE4421C3050DC6D33268CABE633C6E54C13D64973751798A99D8C53A2132E5BBBF8B2961B9E800F01940C700E47E4E1";
        let pk = key_gen_internal(&xi.try_into().unwrap());
        assert_eq!(hex::encode_upper(pk), _pk_expected);
    }
    {
        let xi = hex::decode("f696484048ec21f96cf50a56d0759c448f3779752f0383d37449690694cf7a68").unwrap();
        let _pk_expected = "bd4e96f9a038ab5e36214fe69c0b1cb835ef9d7c8417e76aecd152f5cddebec8a1ac25f03b3643700dbf76eef49a324f93d5042e203f3c70658ad1ad13b917cc1ed23f06a4dd1c543350525a9e2451dfe5f3969b1fa530488cc8903fdeb7718d123b17843f82a838976c6596f4b18e7b1f15ab3c526b90118506f33338d539cb77021f0ea75d424a7d90a9b689d3ffcec54e47d9f2d06f606d35868ee4fc3a038c29e1496592715ea8fb7a8a5e5340d8ba8ac8d81ee38f78161879a0564ded36899e88f6be522e5463810dcdad14a9ec1a9994b1cd74601aefd31c9d3d009653cfb8233c06d89b4c0d8c2560f8f6bbf2b8cdf37483710b87206fcb16f7b5472033c6e9fcd81e05c284c168b82899532782dbf7897aecf7033e85512237371271330ad1b29f613a5de56d54d5d78f50a41725601daaea33ad0bbd8fcb77ad6342df6c2890688bccea1c7d9d92c7c57a5c2196b346718381bc00072a62f8ebc31e6d3bdd99d55d80e17d21163b61406ad4eaa70927e4fa74add922624d964725f11c9b7b52a5f9e3a6ec36e1f17d0ea61baf68ed8c04851a1a82730d39da1ad2e69e38288f55c13f75fc65dec5af6634ade84ee77459453a126f5a5902a806903c7914fbfb25515be9e57aebb8ca258d281e1a06109d85ea687de74a40f14235bd4d7541c05096800c47ad4d7f1554817c962d23840050c3f1c12966e586bcb6e71659168d96e6610ca391970581979aa40e6247b5c1661042468fa50e20e0435c7e7159b12fb3ec2d06dba6aa40030531f48071f645f7838d9faef5ed83ec5676cd4f5aa25e095cecceabc2df851488a5188ef9ef47b75ea42795d73b63800796331688fbf6e0c2fc0a6193c729209e013af51d52d1805b5ef72dda8e7827d38d92a70c4e09f6b0223dbc3e55c15ddb6aa5650d62078cfb6fe30668dd0c283ff320c75cc595ea043d063562058e2d5b80d0ec8830f4f0065acda05b4a132ffc6a06ee439a62de3c46bd6526f7d788c2e0ed47da1081db163d58624ca8702c556807e721224b060702e7d7c49b339bfda25a631cc65a0185bf17f32fc444347bbb5a43ec1e934aaae874a2dcfa1e1371d61f0b6a7b24b8eaf25a9e02f8fa38d9596db39b5d8d4d245469de1eef739200c9309a01346268db557f18ee192640a57f764304e4e0ca56d4cb3dc16be0ace079d6dd6c27d9b0c76a122614b6f423d586ae5bd1a8437b7efd0d69f2890de5d17252512d4c83a1b612217b15f594c7ff1ca5aa13fa8abe19300085d807c7acb4caa80db574fa450684fe643581dbc7901fc564d1d7d4d2b3a8956577100ed62b3804589aa7c30b27060b57586ae95632899fc81a6349ea674938db8facb975bbda16f185c5f5d8d3230d9d60a62fa0a2585f95a6b26d6b3f3ed2555ef2e88e1488508499d6191f2cf05ad22ae44efb958a6c0d172f25732636224f3b855158999ed5e8467ff7ec0c96666798a47aae92dd55e209895d4e9defa1ef257ba07e290433f2deff707948b38b53c34be66becd90327f0894e42bab1033391a365295cdadb7c99fd0e0a4fe22efec6cf2df80895a98b65b2aa03e7cd05918446165b177652dd5baf1e293c940c07bb620b4a99fe98b1a42501181e66f161864758dd146c70e018793351dead59d9347b4e524367b11c8d12d83682a922360c46fc524a5ecdd6aa3ee54c56cd6db930e5283d3009c8d2541811d262d35513c22569ea1198ebb9353d6d26ef3417be145358adf63ff243858a98bf53554a7ed4469ca8b09a76aec882db0c358e76da955ee26dbe17daf57a3a46aa208deaa1ce352dcb09c27067cabd233d5a251ccdc1ac077edb9c";
        let pk = key_gen_internal(&xi.try_into().unwrap());
        assert_eq!(hex::encode(pk), _pk_expected);
    }
    {
        let xi = hex::decode("6de62e3465a55c9c78a07d265be8540b3e58b0801a124d07ff12b438d5202ea0").unwrap();
        let _pk_expected = "ad82b0e363075fd8112f5ef216a52f33f09df7a8c3a0eeee94cfd3dbebe24fe031c69ac7dc807181fdf0fc4e152d8ddc94f33ae6ec23ffa542f48019bb34321dad53ccfce2872ef2f0f21c49eb66ae1bd53987dd2548891236b64b9325cd9fe8553493d304d198c7bf3788054efdf3d6d9debba0e629796e7275fa9c9147a25c9e61041a4dd9b0e3048cfb97a1eadafabef9a67d2bf2959498d9ee75fdaf53864c7a028e2943e3bc64170433f4856982c6d593056030b59fd6f45a6912a6a5e9beff6c8f9e195bc203d3cc94b21c48e84969f2e6851e177af0d041a2136bf63002dd88c84f8e31b312f31480df59ec369ecd5185c5b95d7360778feacba379360d538442691f6b6eb4eb7f42faadf80118d01f68a610a6f6effa5e2206c496070826d79560d7366474800d6d87b7a38f1cace87ef66f2abb92461bc0853d277c45a854c532e23993c4c8d90b9a061e6c01e96eb6b87777ffc2edf45a742b1c93766a5e995823761caa01be5d4611b6e7248196ea5c35b50cf321a9e9e743f11707e6f96c101128d0ab9612ff9ce224fbeda31fb286154ae4c78da09250b0d037867fbdd03d2df68620a96370717f5f92985931ba14e2069062685aabe2c7c504db9fcec1b921c29af3f1be954976a0b0d3943360e1dbc1bde533444d9e6d6709ae5daa12eadc6e462961354e9b947f306d69a827152f17a3ea1d526bf1db2d9f0cd066993f270692c14544d547aa69aa46feace3789b4ba86d7009956edfec237ef075f41a0caa173d45e72402883e2259e0bd99d3dda16152ddf91aabec3408948a6e8a682c0273ae28a477f2d69e2d3cafe992ef5b132470c9662463a2739607b6425ded9a31ad17ce3b487f63a9814a662d6136494791d1cc42cbc7be7cfc1a9800a2a486a07f2ff56659e116c76048c96a46c41ca1051d5b4cef95032b32211177f3d7572fbbdb5904fe86a38afef337fa87f54fa8e0b03bcf85b9c2c25b9a73c164451da1e3a16088a3849d2a521610cf2ef7ee9aa73bb9c34bb90f8cca219514a7136bdf59ea628b7897fa2197de7843f21de531321e87f517125eaba5f5b57c72930b55012b01e8541dd54ef2eaa324a25f1cb7decf7e65a98555799e826cd64434bb5807cf85cef75d552cc0ad080b38c3bd981d2fa01cdaea20251bbe42a0bbcdeb8fca5187ebf3d8b91d0397b890a73b2e7d7ced328de102c74729dc19e41c8fb66d925108647d088b55964c96db593afd259018520c5ffd64040e0daa5ebdae7294d57b4395062a391449ed071aec396e1035f37049b5a9688bba4dd3a8ff00870219e9db0c4f247d03a5543d19072ba0adc7647c94cdf3247299b4374e77020ef15988e7188720540fe03414cff836d638080c8a2a1b6e743152d6f79bc5da618b947a9c0484e9b9eda925358b0130fff2ad41a0f650278f4a3a5b3f77c4b2436c6689161820fbd77e813459393c944bf35dee82814bb2aa50dd345b2fd838107264f11d58ba624cf25fed80dffa261d090b6273c9218219bbd204e6889c8f0b0c3f0ec8889350efac58181369f6987d38793ec1db07cd7f6a9fb1a9a201ee515e49eaf1caab17756910b44d17085ba9ca8ba71b75079a2dd64f5abd74aefb449be6105ef15aedaa8e536489234deee064bb9c14930595b64bbe4b65e26cb5c7a0c33d6439a1c82158ef661547f0a8318fa3f194b866bbbb8863b6407321a1b5122ace9aa7b2eae806ddec7c8138da1afbc930da5468ce09ea9f3a2454cdffd2ddc31c71b1f0f96a9639c7022227dc5f181293a67bc2dc3d61534cca9018145b224fa76545215d0dedd943e323acf94047bc5e13ec2175422c46";
        let pk = key_gen_internal(&xi.try_into().unwrap());
        assert_eq!(hex::encode(pk), _pk_expected);
    }
    {
        let xi = hex::decode("1eaae6bb91b27cd748c402c4111140d5a942cf3c95ff7977f88d2ef515bb26d0").unwrap();
        let _pk_expected = "acdab29943a7515582042304cdda0812c96ae611ea6184ae62f27bde18a95bc9ffc9f4fecd804ac8f0c8da929e41b2ee33196024c7bca29ff2789a290b4f221692fa63429c9d0926d5621ff38d6b932e9cbed7256c085c160faee1f8f07a4edd7117824c32344fb92c395a1276a6c4fd7b8ebb4b5247904ee78a6db3b8f740f437313107e9480a8bb0e975e61a7d177ec71e8f2cdf9c79d388b0f1795fc6c6315aa9912f0f6c7ba03e4a38441e7f7ca87eb7519181223991d96f32182a8f818766d51e8253ccecbfbf577f5d93543a9b4f6759f89c0cecebf0ad684f105d165420d89cae3f19a15ebb6a2200a25737b448f3bdb07e398df21b2e369d88edfc38321473a1122c72dbb3c0dd95963f053aab96d5013948719cbaa459d7a9370d09f05c53aff00ccd2bb141b61bf24808dddf3746665d7955f4ad3e026344c30f0ca088ed6ba8c46ee1ec6dc41fea4af4a077cb075e012ed9c27b96d62ecc725cc08d5a30f176cbe43ad4c92d3e555580e1c3aadc71fa5677f501611cd59664e70fe5e0a103c7f12a7a0f37bdf61277e56246337ce6e299af705843b8b2eefc941fba683220bd4f86b3f3bca2b257f08ebe0ced78cdbe649b3b37f7328b07cf89b2b63427b7e44ab58062a8ef310ffc5208037317041bb8cb331b5c363029b25c39929b81cb8ca2d9ab414abb1eb5d58648c2a171d4bdbd3892cdca42216ce03b7e5b957e64efc71bc97241afeb4e355c3a44db6268dbf1d685d8cee2ecc9acda167b2e55e40da4f6fb6efaa8a6978d1b0daa971d106a3921170b420d06af089e238815e26097145fae313841622f55e066ae05f9b6e9e265e3d9f419a77e9020797713ec4e7e400115f34639c896c9c27635e6a89aac93b070dfdf8b088522ed076b94bbd33706aaa944fdf6c86141767b084c62145359dece67975b4b066e04036036231464e4882dc1b5fb7905f5f2d62042ff023d20b2bb788c7b65ccc155e5b639e7ffd97db6025c70dfecf40e8d9fe0e3f1c804bdc76a6587bd8fe772e3acc4f842621d83b4bff390b8046e020633e019b2bc2f72ebd88cfb8df31e48f48be421522f733c00c724a06c5570e3e9126d5a37fdec7f795c19bf9bd60a8afb75ec88656b95e8ee54f37eab80fe54d8595563ccfe57fbc17252eadd518d3cfde48e9e5649918dab953a3a4cf131f249497d4a35da22b054dd74ab81c21eefb77789062889e593b694877c620290b355b0ca6583dcb87efd679df5272c86f8c529a13b2092dc643f8ba0fd20e3ee36990f52f317d7650199d67404a02a8ffc066e65c38568b9c2eee595f072ffb886f0d43688cca89858f56d42fe45ab1b693602fec04aa87d7247e252cf1c520ef673df40779d3a7b1761ab6c16838cc893d9ca8087a1b726d34bfe34c87abe15abc1a0372d763ddbdbeb26560582c703df807de579352652128ee3019834311e8c77f792e5208901152b1b586a3ce8ad86317a5dc311f61c4183e33dccedafbbb7b30d4a9eaef6d3c99a89706ff229807a72b03ead6830e8b03e918f1ab8bfd61f7f17689833425d2d627579f7aca34af2d872ec0ac72a0d70451e9e5fea782364f8a4e41ea4d4a7fe0809f3bc8660ef31043d32ded78f748daceadb3dbd9b2c54fac2aa82fa193c7fb5249c7f56cd56aa6c7592d4d16668ee9600ae511e363e2a88ef6f5781294fb9a8bc09dc7bef1596cbba855201fcf365223c850fdec43cb45a0b10ce99f63dae79b9cffd61875c2682fd25fb8fe9e6be4d565706070765eb2df03c623371e9975c6af27299fea8bd2a54f8e3139b14624b777ec27b4370490484ba0e1e6df260497269e81c5262442f17";
        let pk = key_gen_internal(&xi.try_into().unwrap());
        assert_eq!(hex::encode(pk), _pk_expected);
    }
    {
        let xi = hex::decode("b585d4eb01085111a172a87688d0032e3381a9e9a35fdd6ef2f8aeb3b40eb5ce").unwrap();
        let _pk_expected = "9398dba05f34ea8e6bac58fd37f83d3e38ceb477f4653fe895a63be478c1dd8328ba5be45acb5cdcf1d4179bdb517442eb4a8974a363698e37e574818137ea4cd8984272fd41f0270bdf82c8ec4535078d70af1c66b3cfde7c7ea8d3fec252ff9f6bf39fdf78eb2d6f4c3bf71c8eeab654854ba0c917f29a8dafe2407eea9d986adf5393443806ed2f6f4d8e48139af331060f1361a535b634e600166417358c4867bf635c4f096cbba84004cf37bc1739e7cef67330a7769e8fece463c08f13946b65b46b042a06419041c63f17509d9811787511961d26463b1a453ca855a39a9191598a93675ac4ec8d82387793517ec37378c1e6f64c040c3b0df2c8ef960db8d0df64077644b39f5ca68b1eed61da95a464a7af30673ddaeec7a8573dcde37fb42d25fa0b9f7c6fed21b3be8a536a6982f1a988ff2b0f034adcf920bff805677e1f9452d7320e8095f22c6e89dd0c333ced96444baa9aa3fd1225733d8dd2ab9be7a95d6892b02818e1962037ed4a984bee33481c3044bb9d915444726196cafeab3289a2fa0ee908ea28f180cebd4f266a121ac74e2d1aae6438e26027313bc446857a56271b9d7e9db9e35964a0a37b088e2b2b67d3cbcf2e72a1a18caff3d3114f1abafc3375dfb7e9709a3bd3d6d77049c3b741b084316241d16d03f40a71f6714de91c33ca34aac05dcbf128f5c673b92c743b71060190e934b4896bb097047ebe74e08b9429823dc4e525f08bf3aede9f202e50846af0330bef16c412493b81d4d547229e73f03133ef723b06e406f8783dc8d47ae753ee1634c3eb801f9a5fb2a80f38e7fccf0f648ef91b84b62a8625af8d6bb98061ade0024ab90ed20efab6c3cd5f4848aa187eaf0b4450e9631c153fc83f62d63422b980bfae2eb0d39d9cdf8818138e7c21b064344a5ff844115de7bd817108f304842c3c166b843e0e144da444805350131dd9c9e08d2459605490859214f0d8225f8f9ea4b09fb90cd1ed1368b13288757c412913a3580caa12f8119ba802c857095fe5b1aec31d75f82af6769ac4fea18d4aa377b435b1b4e38be0419785ccfdd98f8af5c0f542bc1aa18608992f642e0fba633fce569e9d55c59999dd66cf57f239f273507043b3b647fe077b601874603585f9d607b6f8c88a1140be08ab588c67d65a4d9173444ba703ba22f3f3db24d6ccefbf48b3a44d179bbb992f4326d3fd24e3983ca6e2112f4b52af234718559032b3e3faa85feefa40270ddd21a160a424cba945c58aa2e30f42dcc7f86a50b2fd2dfd9d3c12168bc027611bd7edc6e6ce16f2ec4376ad1654bdcdcf432d5fd4f3ef52c84ea81c7f047fd833fa4a8b97fa2c519e066c933c8f89b922722757da301d4426f5ed8b86ea8d3102048f500f94f43f6a40bacf1cdbb5f19162c515aeff43acf1fb9519e13e0b468860ab5cf008e8acb3350430fc739f9a0c833d829052021f2d27d2c507bb65429230f85046f68c4f1920e1a85b90949edd3b34d5d4e622d7c36c9c41b88cf417ceed2970b0b4b4507ac757ce109931cd09b422473ceeb52fc33bce3af8a8b5aa9dcef27be840dd23a5cbfee02baffa9db5228de78f92046b71e465ab888a6877ad5f5d7059baf9e26ebabf126cfd9515486a0763add8f4938704b8d60ef39f89ace3730bdbe654659c26f38f225fb1385c786b35e359ad945301ba576bb870021dd5c07219afab2bcaa670991f461731e7aa2ff6ceb23f36d8ac4cda53631693ba8c7556264d7d53d35bcd79d74868e096d6ad5ad2d27cdd5b9e96f40d20f0616ea2facc8ad016477e5b652b5359e14d309ea7a984f9613b50f2180ceb66d686c29a900657a2";
        let pk = key_gen_internal(&xi.try_into().unwrap());
        assert_eq!(hex::encode(pk), _pk_expected);
    }
    {
        let xi = hex::decode("757249d617ffe21cb99c7af47efcd1909e40dc9a95010c2361f071f60a44ecec").unwrap();
        let _pk_expected = "4773ad1c9cdbd3a70c017084d79f961606156b26da9800cbf90f95c9f2846239aca467bc25a7686cb3169b8dd1e4779e804d08e401f8fa563229538d6ba8218076a1ba954a73ac884d76675b8f05734774249831945ff386b3056d62c53c1c24dc9d3a4e7150cb45001271c8ec4d44d81b012d048e290a9a4a70b22d2f0cd2e49fe24bc173f1d407f95bc81da8b359b8968d253fe627d8bd349946e9e02c061b1558e7dfcf85866d205ed4ccef4978a621798d18f9e5780a282bd655b25a21f28bbabe5ff9cd3b8c4d72ef7cb8afc645790aa8a184dbd65060ff913209b89a83882d17ae2d4e5feb352fca193a6e431be69228f10ab277d2241b3db09866ccac5b6ec884691da332eac1355675637605de2984e79d35900d31e8b9fd260a2e5a2f1787917ca756f90612d812f7a39772a327a18651327d8ea841bf10fbfac7baa3b99c3c737ee8edae1aace83b8a1027510f9aaeecdbecc40e2fd8b17a41d758094b4894b0b669770e465809ebf25232829195d4f64deaac07bf8c1a7353c064f43d618e064c60ce6fd528a79533d0999c35beef8162e3e1bcc9a4bc8c5111820b7d8e8927e3f4a5e79959fd2d4ff33f10d3228b0afec31b1e239510a57fc36070509374519527afa5fa1dbd99e6237e77fde4770fe4eea5f9cdb52ff98aeb1e6e858740a84c2f8cede91679f61a39524879433ce08c3bbc3d6640b240bc9ac9f09bed9c378f1ea21cc1e205e6c1d071ba1c5cfb82a15fe9721632ea573b56c68104c2089e8988226a6e0c8e6c8d9e7389ea43372404872c190e3ed90f0d611c749276fd62b485b5dcb09b1d0dabea94e4b6732681222445b3b4642dacdf5ef6c44ebc017d1965356bde749a7aa69b3e7606ddd917b41c74e867c04415aa627f2e6f6aab75bcdd578a803c96b582281faba7699e48b93fa952e523e3987c17061cce10ed134c598828165c281af1fc560282627c12e0b94e2ab0408dda9b828e25eb073b9b8dcc6dd22d9f710b7cbe786e154096e051a22b37a6a40b31667293584e25db74cbe5a68429cb81f3f98dc13310eb3f7056c8e0a01f348c1eb3945eb3405aa034f054c8ba74fb99fa4d035d07b9c20eddd82fff1c568f22568708449aff3d460fe6ce02f211aa88dc9216a74a9abb9f6d9f57e44118367baf9b364ee96c134f3ef0196ec31c6872760e381b71e87b86570921a4faaaa0242cfae41d4b508418dd5cc6c6a975d2fea9d0ef00c8480ec87f930b1de61c1fe1fbff734cd1771f77fc3d4d0a1a7713c0c58986e941abca173ef138dbec9a43fa17540e7cfc388114178bde17dfd622eb6bff943d0ba742f50c7d00b1f037a960b7b402382d5d34f83388ce8d2a98b3e63ebb413b8854445db806f7f489b0a4c68fbd138b7c2320ab678ba371c196ad907d12933fd5caec9f87041bebcd5ffdfdd0fd10bd61bcf1601f6eb3d235ea169aaa849a0f7a962522153fabf3dbfbeb9799760776407b0199709fb46008760e70a267514a1bd2cf887b418fdb84aa0b025f55e446499c0be9c228f968f02223b2d018f2835dc1b0a9f1dd05a8431dd8278efe19feecbcc033edc7dad22d72179294e18f42442dafe16e8e7fd602a76bbf237ec0225749e5030904392467cfab1d63a2f05d197bb25973542f1e6f5ce37b09490f153083258637b9faefb8b2991ce651ec8d43ab476c0f9038e231b4f3ac23c7b67fd713c2710ac0885d7f52bf7f7462ea8c0548de7d73f0c7f537a1d10db278437ce8f5f74c2ab6749b1b3384dff9bd7e804387ed899de4ded655de16cdf8c6a14047aade021cf1566e22f939d512192e3b639bd54c2b519fe31238232d18e0e5c9";
        let pk = key_gen_internal(&xi.try_into().unwrap());
        assert_eq!(hex::encode(pk), _pk_expected);
    }
    Ok(key_gen_internal(&xi))
}

fn to_polynomial_ring(s: &[i8; 256]) -> [i32; 256] {
    let t1 = s.map(|x| mod_q(x as i64));
    let t2 = s.map(|x| (x as i32) + (((x as i32) >> 31) & Q));
    assert_eq!(t1, t2);
    t2
}

// fn key_gen_internal (seed: &[u8; 32]) -> ([[[i32; 256]; L]; K], [[i32; 256]; K]) {
fn key_gen_internal (xi: &[u8; 32]) -> [u8; 1312] {
    let mut rho = [0u8; 32];
    let mut rho_prime = [0u8; 64];
    let mut k = [0u8; 32];

    {
        let mut d = [0u8; 34];
        d[..32].copy_from_slice(xi);
        d[32] = K as u8;
        d[33] = L as u8;

        let mut seed = [0u8; 128];
        sha3::Shake256::digest_xof(&d, &mut seed);

        rho.copy_from_slice(&seed[0..32]);
        rho_prime.copy_from_slice(&seed[32..96]);
        k.copy_from_slice(&seed[96..128]);
    }

    // Expand matrix.
    let a_hat = expand_a(&rho);
    // Sample short vectors s1 and s2.
    let (s1, s2): ([[i32; 256]; L], [[i32; 256]; K]) = expand_s(&rho_prime);
    let mut s1_hat = [[0i32; 256]; L];
    for (i, cv) in s1.iter().enumerate() {
        s1_hat[i] = ntt(&cv);
    }
    // multiply a_hat and s1_hat
    let mut prod_a_s1 =  [[0i32; 256]; K];
    for (i, a_row_poly) in a_hat.iter().enumerate() {
        for (k, s1_col_poly) in s1_hat.iter().enumerate() {
            prod_a_s1[i] = ntt_add(&prod_a_s1[i], &ntt_multiply(&a_row_poly[k], &s1_col_poly))
        }
    }
    let r = prod_a_s1.map(|v| ntt_inverse(&v));
    let t: [[i32; 256]; K] = std::array::from_fn(|i| vec_add(&r[i], &s2[i]));

    let mut t0 = [[0i32; 256]; K];
    let mut t1 = [[0i32; 256]; K];
    for (i, t_vec) in t.iter().enumerate() {
        for j in 0..256 {
            (t1[i][j], t0[i][j]) = power2round(t_vec[j])
        }
        for j in 0..256 {
            assert!(t1[i][j] < 1024_i32);
        }
    }

    let pk = pk_encode(&rho, &t1);
    // println!("public key: {}", hex::encode_upper(pk));
    pk
}

fn pk_encode(rho: &[u8; 32], t1: &[[i32; 256]; K]) -> [u8; 1312]{
    let mut pk = [0u8; 1312];
    pk[0..32].copy_from_slice(rho);
    for i in 0..K {
        let t_packed = simple_bit_pack(&t1[i], 1023); // 2^(bitlen(q-1)-d) - 1 = 2^10 - 1 = 1023
        pk[32+(i*320)..32+((i+1)*320)].copy_from_slice(&t_packed)
    }
    pk
}

fn simple_bit_pack(a: &[i32; 256], _b: u32) -> [u8; 320] {
    let mut r = [0u8; 320];
    for i in 0..N as usize/4 {
        r[5*i+0] = ((a[4*i+0]) >> 0) as u8;
        r[5*i+1] = (((a[4*i+0]) >> 8) | ((a[4*i+1]) << 2)) as u8;
        r[5*i+2] = (((a[4*i+1]) >> 6) | ((a[4*i+2]) << 4)) as u8;
        r[5*i+3] = (((a[4*i+2]) >> 4) | ((a[4*i+3]) << 6)) as u8;
        r[5*i+4] = ((a[4*i+3]) >> 2) as u8;
    }
    r
}

fn vec_add(a: &[i32; 256], b: &[i32; 256]) -> [i32; 256] {
    ntt_add(a, b)
}

fn expand_a(seed: &[u8; 32]) -> [[[i32; 256]; L]; K] {
    let mut a_hat: [[[i32; 256]; L]; K] = [[[0i32; 256]; L]; K];
    let mut rp = [0u8; 34];
    rp[0..32].copy_from_slice(seed);
    for r in 0..K {
        for s in 0..L {
            rp[32] = s as u8;
            rp[33] = r as u8;
            let z_q = reg_ntt_poly(rp);
            a_hat[r][s] = z_q;
        }
    }
    a_hat
}

fn expand_s(r: &[u8; 64]) -> ([[i32; 256]; L], [[i32; 256]; K]) {
    let mut s1 = [[0i32; 256]; L];
    let mut s2 = [[0i32; 256]; K];
    let mut rho = [0u8; 66];
    rho[0..64].copy_from_slice(r);
    rho[65] = 0; // rho[64..65] = IntegerToBytes(r, 2)
    for r in 0..L {
        rho[64] = r as u8;
        s1[r] = reg_bounded_poly(rho)
    }
    for r in 0..K {
        rho[64] = (r + L) as u8;
        s2[r] = reg_bounded_poly(rho);
    }
    (s1, s2)
}

// q = 2^23 - 2^13 + 1 = 8380417.
// returns an element in Z_q which fits in 3 bytes.
// this is a polynomial in the NTT form.
fn reg_ntt_poly(seed:[u8; 34]) -> [i32; 256] {
    let mut j = 0usize;
    let mut g = sha3::Shake128::default();
    g.update(&seed);
    let mut xof = g.finalize_xof();
    let mut poly = [0i32; 256];
    while j < 256 {
        let mut s = [0u8; 3];
        xof.read(&mut s);
        if let Ok(z_q) = coefficient_from_three_bytes(s[0], s[1], s[2]) {
            assert!(z_q < Q);
            poly[j] = z_q; // mod_q(z_q as i64);
            j += 1;
        }
    }
    poly
}

fn reg_bounded_poly(rho: [u8; 66]) -> [i32; 256] {
    let mut poly = [0i8; 256];
    let mut h = sha3::Shake256::default();
    h.update(&rho);
    let mut xof = h.finalize_xof();
    let mut j = 0usize;
    while j < 256 {
        let mut z = [0u8];
        xof.read(&mut z);
        let rz0 = coefficient_from_half_byte(z[0] & 0x0F);
        let rz1 = coefficient_from_half_byte((z[0] >> 4) & 0x0F);
        if let Ok(z0) = rz0 {
            poly[j] = z0;
            j += 1;
        }
        if let Ok(z1) = rz1 && j < 256 {
            poly[j] = z1;
            j += 1;
        }
    }
    to_polynomial_ring(&poly)
}

fn coefficient_from_half_byte(b: u8) -> Result<i8, MlDsaError> {
    assert_eq!(ETA, 2);
    const MOD5: [i8; 16] = [0,1,2,3,4,0,1,2,3,4,0,1,2,3,4,0];
    if ETA == 2 && b < 15 {
        Ok(2 - MOD5[(b & 0x0F) as usize])
    } else {
        Err(MlDsaError::BoundedPolySampleError)
    }
}

// generates an element of {0, 1, 2,..., q-1} + { NTTPolySampleError }
fn coefficient_from_three_bytes(b0: u8, b1: u8, b2: u8) -> Result<i32, MlDsaError> {
    let b2 = (b2 & 127) as i32; // set the top bit of b2 to 0, such that 0 <= b <= 127
    let z = (b2 << 16) + ((b1 as i32) << 8) + (b0 as i32);
    if z < Q {
        Ok(z)
    } else {
        Err(MlDsaError::NTTPolySampleError)
    }
}

fn power2round(r:i32) -> (i32, i32) {
    assert!(r < Q);
    let r1 = (r + (1 << (D-1)) - 1) >> D;
    let r0 = r - (r1 << D);
    assert!(r1 < 1024);
    (r1, r0)
}


// NIST ML-DSA-44 ACVP KATs.
// https://github.com/usnistgov/ACVP-Server/blob/master/gen-val/json-files/ML-DSA-keyGen-FIPS204/expectedResults.json
#[cfg(test)]
mod nist_avcp_ml_dsa_44_keygen_tests {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use crate::keypair::key_gen_internal;

    /// Reads a text file in the format:
    /// tcId = <hex>
    /// xi = <hex>
    /// pk = <hex>
    /// sk = <hex>
    /// (blank line between test cases)
    pub fn read_mldsa_44_nist_avcp_kats(path: &str) -> io::Result<Vec<(String, String, String, String)>> {
        let file = File::open(path).expect("cannot open file");
        let reader = BufReader::new(file);

        let mut results = Vec::new();
        let mut tid = String::new();
        let mut xi = String::new();
        let mut pk = String::new();
        let mut sk = String::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                // End of one test case
                if !tid.is_empty() {
                    results.push((tid.clone(), xi.clone(), pk.clone(), sk.clone()));
                    tid.clear();
                    xi.clear();
                    pk.clear();
                    sk.clear();
                }
                continue;
            }

            if let Some(val) = line.strip_prefix("tid = ") {
                tid = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("xi = ") {
                xi = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("pk = ") {
                pk = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("sk = ") {
                sk = val.trim().to_string();
            }
        }
        if !tid.is_empty() {
            results.push((tid, xi, pk, sk));
        }
        Ok(results)
    }

    #[test]
    fn key_gen_avcp_nist_ml_dsa_44_tests() {
        let kats = read_mldsa_44_nist_avcp_kats("./kats/nist-acvp-keygen-kats.txt").unwrap();
        for (_id, kat_xi, kat_pk, kat_sk) in kats.iter() {
            let xi: [u8; 32] = hex::decode(&kat_xi).unwrap().try_into().unwrap();
            let kat_pk: [u8; 1312] = hex::decode(&kat_pk).unwrap().try_into().unwrap();
            let _kat_sk: [u8; 2560] = hex::decode(&kat_sk).unwrap().try_into().unwrap();
            let pk = key_gen_internal(&xi);
            assert_eq!(pk, kat_pk);
        }
    }
}

#[cfg(test)]
mod misc_ml_dsa_44_kats {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use crate::keypair::key_gen_internal;

    /// Reads a NIST ML-DSA KAT text file (no blank lines).
    /// Each test starts with `count = N`, followed by `seed`, `pk`, and `sk`.
    pub fn read_mldsa_hedged_kats(path: &str) -> io::Result<Vec<(String, String, String, String)>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut results = Vec::new();
        let mut tid = String::new();
        let mut xi = String::new();
        let mut pk = String::new();
        let mut sk = String::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some(val) = line.strip_prefix("count = ") {
                // If previous test exists, push it before starting a new one
                if !tid.is_empty() {
                    results.push((tid.clone(), xi.clone(), pk.clone(), sk.clone()));
                    xi.clear();
                    pk.clear();
                    sk.clear();
                }
                tid = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("xi = ") {
                xi = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("pk = ") {
                pk = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("sk = ") {
                sk = val.trim().to_string();
            }
        }

        // Push the final entry if the file didn’t end with another "count ="
        if !tid.is_empty() {
            results.push((tid, xi, pk, sk));
        }

        Ok(results)
    }

    #[test]
    fn key_gen_misc_ml_dsa_44_tests() {
        let kats = read_mldsa_hedged_kats("./kats/misc-keygen-kats.txt").unwrap();
        for (_id, kat_xi, kat_pk, kat_sk) in kats.iter() {
            let xi: [u8; 32] = hex::decode(&kat_xi).unwrap().try_into().unwrap();
            let kat_pk: [u8; 1312] = hex::decode(&kat_pk).unwrap().try_into().unwrap();
            let _kat_sk: [u8; 2560] = hex::decode(&kat_sk).unwrap().try_into().unwrap();
            let pk = key_gen_internal(&xi);
            assert_eq!(pk, kat_pk);
        }
    }
}


#[cfg(test)]
mod keypair_tests {
    use crate::keypair::to_polynomial_ring;
    use crate::params::Q;

    #[test]
    fn test_it_to_ring() {
        let mut a = [0i8; 256];
        let mut b = [0i32; 256];
        for i in 1..128 {
            a[i] = -(i as i8);
            b[i] = Q - (i as i32);
        }

        let r = to_polynomial_ring(&a);
        assert_eq!(r, b);
    }
}