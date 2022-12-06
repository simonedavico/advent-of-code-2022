use std::{char, collections::HashSet};

fn priority_for_item_type(item_type: char) -> u8 {
    match item_type {
        'A'..='Z' => item_type as u8 - 38,
        'a'..='z' => item_type as u8 - 96,
        _ => panic!("Unsupported item type!"),
    }
}

fn find_common_item(buckets: &[&str]) -> char {
    buckets
        .iter()
        .map(|b| b.chars().collect::<HashSet<_>>())
        .reduce(|acc, set| &acc & &set)
        .and_then(|set| set.into_iter().next())
        .unwrap_or_else(|| panic!("No common item found!"))
}

// part one
fn sum_of_priorities() -> i16 {
    RUCKSACKS
        .lines()
        .map(|rucksack| {
            // assume even number of items in rucksack
            let half = rucksack.len() / 2;
            let first_compartment = &rucksack[..half];
            let second_compartment = &rucksack[half..];

            let item = find_common_item(&[first_compartment, second_compartment]);
            priority_for_item_type(item) as i16
        })
        .sum()
}

// part two
fn sum_of_group_priorities() -> i16 {
    RUCKSACKS
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let group_key = find_common_item(group);
            priority_for_item_type(group_key) as i16
        })
        .sum()
}

fn main() {
    println!("Result: {}", sum_of_priorities());
    println!("Group: {}", sum_of_group_priorities());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_priority_for_item_type() {
        assert_eq!(priority_for_item_type('a'), 1);
        assert_eq!(priority_for_item_type('z'), 26);
        assert_eq!(priority_for_item_type('A'), 27);
        assert_eq!(priority_for_item_type('Z'), 52);
    }

    #[test]
    fn returns_sum_of_priorities() {
        assert_eq!(sum_of_priorities(), 7428);
    }

    #[test]
    fn returns_common_item() {
        assert_eq!(find_common_item(&["abc", "cde", "zyxc"]), 'c');
    }

    #[test]
    fn returns_sum_of_group_keys() {
        assert_eq!(sum_of_group_priorities(), 2650);
    }
}

const RUCKSACKS: &str = "BccTFfTPTsffdDDqsq
lGGLQwFhDgWdqvhW
wbLNjGjlwLFrpSbllrHnHHRmmJVBmZJRRVcBTc
vvGLllBBLtllJnJFMZNjFcNG
bdhrhTgmhRrpLJMMNJgNffnf
qhRmTpzpzVzmTTbmVhWWWpVvDtsLVlwBVHvSsDCvvBDl
sLlhhthVRndBZzwBdV
PfmsPsqsGFqrSQpqBDDwpddDDM
mFjvSFrjPSvLJWvbRssN
gWnWQtMMDQbQvMDjjcwsvqcwsSSqZq
HzPJTtmtZJJsqfBJ
hCFPrNNPVNzFtQhdQMWRhhDd
sNhmsQZdNdsztNpHGCdjcBcwCvGvCv
brSlRfFFppHwfTpf
MbRlRpqMnqRpMhVtsQQQVWsM
HsdttdfHrHrwdhftMHMSDnJFwlQSgQZQZggZSW
LjCTqBCmmmTqgFQQFQjlnnDJ
GCTLBCmTBDBDPTTzqLCBpVcctcHbtbrcMMsdsdtcPtdb
QnJLdNLfLRQjpLlPflfQnlnswRvwsHVWVHWVVDZwWHcrwV
zFbgtqCBhtgBhsDZVDNHwvDv
zgCGCFCFGtMpJjlGTnlnLN
ZFhJZbvZVmFpFnJbnZFbLnbHPrjjQdBdCrMPQMQHrrrrVC
gTzlqmNlTflmtTRHdDPqMjHDMrjdQd
wcRSwglcNSwgWmNzwNlJZLpphbbnFhvLWhnphb
lRRrcbRfQmwwBLSLlvjjSvvFtS
WbghzhHWbsdHhVtGLVSSvLFJtSGC
WWbWzPHDqPfBqRZrmqnZ
frgLHMcHLrSZHPHcHrPNmMJtTCTCbJfhdwVbfJwwTbdd
DpDWRjQvqjDGFWhdVwQCnJgTnTbT
vvqggRsRpcssPrcsrN
MccbcZjmbbNSbjllbRqrhstVTMtqRtssTW
qJdwCLdCzQQzwHLQLTtsTtsTRVTVWWTJst
GdnpFHzwQCdpvpbccPbSbq
CbQhZpTbTgMMgptzrdGtGzlVNlrh
FWvmmRHSmjqnSjqnSjPqjnmNLGVGJlGWLlLrLWlVrGzNGW
FwvvBjmBnqmVvFnvnVPMDspDQTfZMCMTgDQTCB
VmrsHllHVLTdZtRclL
bjwMNQGCjGjjPwpRQnRLTtRQFvcdvt
MbWgjMDpNLMjNpwGpjbPWgfsHsqsHSrrHHJsgBSBzB
HFlwVhfLBZZLTLFHwqWCqWwQpWwSpqWC
PttsNjdtPtcmcDTnDdtjjDMpCCCCmSmqqGqmvMvbCqpC
jndRrTtPdgNPPnLrVhVLHhVJHffV
NJZVqHNNNJNqCdqZZVjtzCDMtzrtrFhhMFsPCS
TbwgwWRdQgfggQgvMPDSPszhSbSDszFs
QpRmWglwTQgGWTvGRvQRdwnjNNqqqBqVlcqVVNncVVHV
gqBFHLFDNCBClHgbLFbllLggVSTWJVWWpjvJpSSWJjBjZvjW
mczdshQcsGftdmcwcfhdzQVrhJZJTZhTVTDjpWpZvvWT
twGdRtcGnRgDqFRC
fsPjCjgRpwjPpsGgQwPfSZcdSZdWVHzSzcdzHRWz
MBLTTBMvbMmvJTLvTDvczNdZNHdHcJNzNqSZVG
rTlDlbBbrsjCCGnG
FsmBPtzHdmmvcvdWpcWCvM
RMJGjGGJJGLDRNJJnfGRRSvplTDccrTwSWClvCrwww
nnRqNGJLVLRRZNNZzBHMHVFVmzBFbhmb
MmTSpBMBCCMsbbrVvwwSLb
RffNRFRThtqHhHHqZNrLVvwQtsGvssdvvsGb
DThHghZfgqhFWRNHhhRWqMCnMzzznlPnBlgzMpnPCz
GhlMNMdpMZHHhhRLLjqSjjqvvmSh
PcTJDBDcBnBbTFcDwnPTQSLqLrqvWjwWLqRSvsRmWm
PDcDJbQTFBCBcncgQPcDgnHgVmppHVMddmZglzVHdMmt
BqFJqJGpBVnJqnjjmwvrNwVPHPrd
ZMbQmhWstZScWjdswwwHNdzHvj
ZZtQCtgZQWSbMMhmMSWQfGfGDBpRJnJqTFFJBgTR
tVcPGGqwgJPqtJtqZZwcZffsfrcNWBnSWWFrfhWBnz
dRMCMQLvNssBWFFR
pvjjQvmDMpHLdPPtgPPTssTHsq
WDBlFBbGdmBrqWjhWcqZCq
RPSRPtncngwgwzhCwghC
cssVsPpVncQMRsVQpRPnRcfJfTTmTdDBLbFlDblTDDDLJf
CZgCCzgzsCDZDzbbBclgvcjcnnjFBqgv
VLGThLWhWdThlWRHVTLTTjNjjdjrnBNBcrNcqmqqcc
VTRWLhpLWHRMPGRGRplVhSfttJpZzwstsbDtwbJzZt
fqhZRLhwZwVSLbCMCJdJCHQGZWdW
ztvpjtpcvgzppPvjvPlDQLnMlHDGWnQDLQWDdJ
PpsPzjPzgLrjzBgLggzrmrVSrTSfSqFVFmVhffFm
qMFDRLNRRJJphbhSgStpptbj
rZrPZZNCCCGrlfsCzBbgstBwtvbtvbvBth
CfVrdndzZGrfzrzdrGddPnNCFMRDJWRDDnRWDFqFDJJFTJJT
dMDbndBMQWnnbDstnMbMQQwspFgsFFjRrqCCFrrprqrgjc
SlLzZmHZSTVGSPVmSPlSmrpFvFjjpgrpgFcvrcFcWG
PHZZTmNVHSWJlHPNLSzPLLHwDBJtbDbnnnMffDbwtnMndQ
tHBzNCztLBRBtrjvSjfnjvvzfpjj
gQwWqlnqWqJgJnDplfflddjdsfdpFf
gcwwmnDccTPWWgJbZNNbZHZCRRTrLB
ZhjgtrJNfDNpqbhqQmqpmb
cFLcwGGHwcGdwFCQjnnnVcmqmjCp
HvvlzjWHjFWTggWStDrDZT
GPjjQtPQbjwWqrmnsjmnqn
SNGdfLLGdlldZSSRWznFsNqFRnqsWs
DdMpMfZMhfZMpDfbDQQbGQVgccgcVt
hHGGGTlddWGgpRdcvwDCDwzgLJzCwzwgwL
MFSbZSnnFJWwQBNMzw
tSjrSFSfqjqqttPnssqjssbhGmWGhcmvldcmhHRhGRhTRf
qdBjBTNndbnqnLmtZmZvvtLvHd
hDJCpfnGhJfDPzGzzCnGPmpZZVLZvHttZHQLHgZLHt
JDrfCDJhGMhFhPzMrCCnrGSwswswFsblTqRlRwjcjRsqBT
tJPRSZCSJJCnmvvvQMrpqLVwqLqMcCCM
hhGGGfsdfTGlfggjMLVVFsMVwMMbqbLF
hdTGGhGhdhfhhwlgNfdhZnZtHHRNtZHnnZmHHzmn
HlgRZglZDWZgfVbdznHddTvV
MhShhQMSFShFPPQplMPmwppwbzdtVbFtfvfbzVbvbTntzbVT
lMwpmJLLLNGwBrcZCjrGCGrD
qjjWRLjNjtGRRWTCghNwsgwcbwmC
HPMBpVGMPMvvVBPswmhTbCwgmsVThT
vDSMSvflBlHpvMMfSSpMFWZdtZWdGWGddqfFtRqQ
ZmnGQfnZgdmRGQGvgnnmHCbbchhpMLrpcbLpdLpzbz
NBFPBWFsSVNJlFqLDLwcwrPLzzCwhp
qlWJqqWFJJjNqJWsFVsVqqRCggtmvvjQZgQnZQmvmHRt
RdCsJbdsVJtRvdzBzrBcjzMTqT
wNSNnnHhnwhHfBHqHjBDJMJr
GnSPLwlFwwLWSwpNWLSNpVZtvPsVJsmRCmtgsvsRsv
GPMwMMPCBPwBGsCGGWqBsslQhVQFccphvhWpmcFhVLmV
rbNnHLLHfHfZFfQQQZml
DzSDNtrztbgCMMBLLCsBts
rTtrVbrrhbbGGhbbbbRqccdBcdBcvRvBSRRV
fDqQDqLFQgQgZFMlFNRgvCNvRRvHvRBHHR
mfPqmFsqqsbJJtGtbt
vNHvgsSGSFDHvspvtSGwJwJNJrllhwhcnwJlwm
dqWdWfQdqQVWwnfMnlcfwzMs
VqQQqBRTqsBQWLppZLpFSHZbvvDt
gdcldHQlQndnHMzCjvCFrzjSFtbLtv
NJGmZZJZGTDsWWJNmDVmsCrSvfPrvTtSPbFFCFFvCT
BDDsmJZVBJwBRsGGDmZNBGqMdnghhcwgbngpqqMgnpgc
VhhvVwmvmwTPCwPwmDRgDCsgWSnfRMSWSM
ptHZZtlZzqbZttHbzrFqnDSMShMgSRRMngDWgrGh
hBttqBhBzlFhtHvTJQJTcvBQJPPv
FjfzfGjsjBfQfMLBNg
VlrppVwrpVSdScwTjVnCCQLQNCMBhWgV
jvtSrSjjtSZpqtHPDPJqRzGR
ZSmbSDswfCDDHBFFvWHJ
jcjcVjltntQMltnVrdNnNjdQgFzWHqzBWGWQvzHwgqqJGz
nccLllwwhLCCSLTmmRTP
HBSnnJSfHvBfNWMNrvnMrZlblFbsbHwsPFVHTsHFls
VLmDLhRgRbZFRwjZ
zQDmCLLDDLGttGGgtLvvVSfqJBSVftSnrJnN
CgGhbgVMNgVVbjrrtcfTDTfvTscrTvSSss
qqFzFBzsPZHmddmsmvWSnHJnncDfWnDJvv
ZwdPqdqpdPlPwdRlsdBqBMjNQjGVgbQgQhGVCCtgbR
CGFTTMLGPgmPfGfCwTPSSFNQDLNQnscQNccWdQLqvqds
pbJlzhZRHtjrbrbjHHrplRWVNsNvddZNWDQVVcnQVscs
pzJtlJlBhJJHJjHhfPfTTFMDmMPBMSgm
gzCBPDDzgvLvgPLgNThPlVZccJTmrZNV
sndSnpsdMSnRMRpjShhNJZJhJjrVcTljTZ
SRRsdnGwRSpptnfMSSpdQtfdWqvbwFgbDJJzCzqqWqWbLWWz
BDnsPDlmmwcnCLLLwPtFTtTtFRqjRrrSVFqn
dfhhzMGzWJhpMWhHWggTHJVFVSSqbqqjdRRtvqqvFrtF
JMGfHffhQhQTNcTTBLCQmm
bfZBvvRRRzFfFFLRvFzZCcQlScchLlGNhSQGGVQh
mwJqTbsHmjbTNcNhQGGJGcVS
mPTgtsnPjwHHmmmbbRDgfpdgBpzvZD
lSnRStHtTZdjrHjnqJglbqgchhDCPCPc
swFBzvBNLpBBsvszvDJhCzgDDCgbbJQzDq
VswvBFmvpSrShmRRMn
vWBBSrWnZfCWVchwhbcjVN
RdHQQpRPJZLTtJgNcNGgbhGh
zpzDzTdqQRqRzzlRDsBBnBfMmCCSZDBS
ZtGSZVpPDtVbQjbwBDzbbL
WcFvTFnTMnnMcnhmQhmhBbBQzCQjVb
RRWsgvgnfqgpGVNg
WJTrJJCzLqCqBTWLsCCqzmPPQrjwHQQGpwGHHmRPVw
bMvnDFnFSbSlGgnpmGQVpRjV
DZSdFZdQZZclsscWqWLhcBhs
PzLlRRNjjRQzvPNQsvddlZfchhWWZJHSlhChDhHhcc
MMfngVwtpVMqVrwrMBgmtGqCHDWqJSDJWhHHSSSJJHHD
tBgnMTMMrgVmrBwMmGfnNjjsdLbbbTQvjNTNsQLz
ZZBZRmPmgpgZGLWLQWslSWmLQL
DnHJJjzqrJffrDnHzJjnMbQWSltttSbStvFQSstSvCLF
MzJDnfzHwHlljJJnqrMjfPGdBRpRBGBRhwVdGGRTBG
CnZCpMFNnFvvNdpHVrWghgtFVFlLRWhh
cBsSBsGcjGcJZDrggtgtVDVrgWmh
wTBJbcbzffJbZccjSbSjBfccNCNMPqqPPdvnHvTvnMTNdnvQ
vMgPmvQmWDMpGpjBbMMH
CcVJNcdNgdhtCVpjBBRppfRTGbph
sFsgNlcdFlJFFwFstNJcvzDWZqqWvqLqzLzmzDwQ
LLVLVsPPVVPCLLrjCNNNgmRdJNdCdfMJpB
DTZZHTWbwwpWbSWDBmJMSFFNmgRRBggf
ZvzDWqDnDwnZTpzZTzWvphPtqhQsltVtPhPhsQrPch
zDgWmDgrpCLmwgWTrjlJBQRJjbFGrcbQ
hSMvqvHtqsdVHlJDcclMBjRMQJ
sqvHsSstSdqhVVvZdqVHZDgwWpZZLLmfmmwfNfWCgmLg
SQWcTnWVWbZWWBcVPnZVbnrNrMFMdqFNqdMqqFhrDQvq
plGLlLGpJLhCGrRqGDDDrdzNGr
wgflHCCCJmpLjCLHtjjgLCtBZswnsTBPVPVsBcPTbZBBSh
vvlMQvvdjdGtVCTJlVJVfJ
FLrFqwwZgNrFWqZwgqrZBLWcBTtppztVbfRJztJbztfztT
WNmmFJwwrFFnNmDgmjdGQMdHMsPvPjsHDG
hGmZHdSRdMmhMZSHlvbTvRbRlVtCTlCR
znnfzgPPDpPfDcgnZTJvJNCZbJVCcNJV
nrLLfQznprrppgprWrnPzQzLSjGsZmHmhBdsqWhdhBMWhdqd
zhtNFSFwRFLCsNrNNBdl
BmQBPjDpBTDgHllgHc
npBjjpQpjGbMnmPpjPQWpwZhtbzJfhwvwtSwhhFFbh
PZcZbcPlbSprcQbbdCwWRSttgtgvWfjC
TGVLVHHmTVHGDTDnGDhgWjwvCjwwwRLLgBjWBR
CHTGsnHVVcJPPcNsNJ
tTqGSSGPGfVfTpqGTbbcVWJLdjtvdzjJthCjlhdlzQ
wwFBZMmZBmgnjzlCWBBjBLjv
rNMsDZnMMWSfDcDWPR
vLzbsczhLmmnlNvrNQHfWd
SMSFqMwjFFDVSZwVTMDjSQlQfNlRrQRWdQfRrWrqHN
CPTwGZDTFCPSjFTSPSFbPgpLscPczmcBLbfgpb
QnQnpFjsbFcSSvCMNvqVSrqq
WfzfTfzzPgHTfwfWtgRLMJDvjMmMVtvDJJCVtqmC
wfWRgPzdgRTWBBWHPBHHBRLTlcZdhjnbZQcGZGpsnphjshbG
pHzPTsBHzqqtQCZZshlWjf
DFFbnvJMDMljjtQjfCbQ
dgJFDGwgmGlMSggGdgdDDlvLNTzpBzLzzwpTBLzqPBczLT
jccNVNdwnclRwlbwlVjdcpJSpGpSllBHgGHZpJpppf
mDThTmsnDsSBpZfmmgBf
MrFrhCshqvWvnWzTWQtzVbRRQwQRNQjdwQ
tBnLJfnQtzRCffmNjSRjZjNZSRrP
dVdMVMvMghHzPhzZhHND
dGWGgplWGVMdMMzCTsbLtLTCLpnT
ftNfNDdSBdrMTdrjMM
cHgHGHzGgJhrPLqSrrJTqp
mVFnQnhQGHFznFhBBbSDRBlSVBBRfS
VqqPBPcPbQHgfrrpcSDR
tpMnsztnGnthhzTtGTGTzzWgJNDlMfSRlRDgRNRrDRHfrg
zWtTFWzTwphChnCzFhzWZGGvBvLqmVmbVQqjqjmBPqBLCP
ZZgZnhrmwmnmgmvrghPmgTGcTSGSMSldgcCQCSqW
LzFLDBfHzHCCqCFGcSlS
HBjRJDLpHpJsJVJqsnhP
PqrqmvmrwzznnPDpjVpDLfDtPGLt
sdRhRWFhShhFccZZsSsNbsNcjCtLpMVWCjptGCfMftBDLMtG
bVZcsdlhdNSbZRSshRcbbqlmnqHmmwzrJlzzgQlmvz
pnrcNGqmrGqnchGhqdWdTlldtQtlMsTq
DvSLgzLSMfbgggCLCwbSSLLtfsWTQTsWllssdltRQQtttQ
bCCzzPPgDPjPvwSzDbwpVNnpnjjrhpnVZGMhcp
jWbGtDdqCqZjdHwcwZMBVQmcvZ
PnTflPRRrlgLTTRlTzFPPQQBcNvHBncQpHMwHNBMwm
fFFRrFLJgRcJglgRzTzrLqWCjtqGGDsjCjdbGdqdhJ
GwbvGqMsDMbpMGzzgRzgpBLjhcch
WFTFNZTZSCcBggBFcrss
CJWWlsWlCtqGJmMGwJ
CGCVhprTrthCZTCNtVGtZDZNdlPPdPwmmvrcbmPmdQRvQWmw
LfzLzssfgHjLFjFLfjMfHsLHmPwdcQWQQlscwlPdQclbPvlw
LBFHjgMzqqjfJqLMzffHzqgHhhJDZSGVDVChCDDpDNNpNtDW
TbzVlmNTVVtnTSWNwDDrpGcwdp
fQQMFbhCfLgfQCsdDcHpsWpdSDsGrr
ghvPLQMfZhjjvPLhbQFQBZqJlTnnnVzRmtRzlmBl
JCLLLwVDwCQsNwwJHmfrMZpMfMMrfPQSMZpS
WlFlzFRnznthqWRGbMpVMbbMPtMjMj
vTnvzqllhdhqTwVBLcJHmmmC
tczhtcJJJbtclWrtJBWJBtJtpqPRSPfpBRgqRfPmpRqddSmM
HCvnsQLNCQwLnDsNHLwQfPSSpPSMfnPddRMmmGpp
wDQwjNwQNHjTHNFDCNmCFNWtWzhJbWbVrhtWccVFlrlV
sPRpCndBCGpCGHttSdvTbWvgdjST
wcmDwqcwmGDTNvjWtrbSrc
lwlVLVGqZGlLzVHHBBBCHBHRPCCz
wdmhffzzphrjqtzRbrrq
CRgGTGTFssZsllHNBlHsFJRjcrtjtPDPcbCtDrLcrjctrc
NHMRHTlFgGNwpQvMpwVvww
MPLJNPqmFWmDFjGS
nbsZtwbZlbZlGlFDDMpVlF
hvsbbbZtvfhhRZbZsfzMbMZbqcrqdrNrNqLgqhrLhJgddLNL
SnMLpRDGlZSZNlnMZpCwjLwzFrHBWCFWBBBr
TttvvtbtVcsJtRsvtQdzWJrBjCCHBWzrFFhCjC
VmvsQdgPbdgVTvgPMSDfPPZfDRRNDNMl
LjngLCNhDNFNhFDhcMqrqqZMcSZnHTMc
PPJwtGlfszGwWtzwQJBPGslJSqRTHZvgcRRZrMSTMMTtRTTc
llPPJWzQPmWmVNgDbLFCLb
NPFlLNBLprpdmmdPBmJnLrdjMVDjMSdqgggQTVDqWMdqVs
vZTbTZRwvvGRTRjWDqSqqQgDGsGq
vzZcfRRZbwbRHRtwZCChBmFNtJBPLBlBJJLpmFTP
pmvZmmTjQFfnvPPHHv
SczhzfbsLNhfccNFsWFRPrDnPDnVnW
BtBbSdtzLBwSLwBmTZpQMpZmZmfZ
PqPQZqtQQLDqrnqdjqdVwVbz
MGRGWMgJHGlRRHfSwfzCfCVVQzbwjrjn
JsmHmSJJmSMWMlTWQBFhLTvhDFZhBQDZ
sbgbbFGTTFNMbMNFWrjsrvWzHWPzPPpf
CVmhVqSqCZmJQhPpHzvZvtzWHjHP
CnmdCQCdnFTnNgRpwT
blZjhbZWVttjWjWLCLVVZCZQjMDQHsBsBQfMDQwjHDwBHH
NdcJdFcJqgpJpNnDQLMFsBnLSnnS
LJrJdJrzvdrrpcNdNcrVlZWbZzZVRhRtVlPttC
mhRtNNtrtBQQrtrBBmQlZwHHqHZSVHHGshSVDwhS
gpdPMTcsLscMccTpbLdHSfGfqwHZDHHqZqHZ
zbzvMpPLppLzLMjTBQRmssjBWRQjlmrN
VzzvggdvFdmffwmGpd
HNbnJTRTmCwwrRpR
LlTnWhLlhLJmLmtZtPcPcVFFPSZgZt
jHcZjHlHzLHHnSNSfL
pQWRrwPwrRWBWBPWBRrpdPmzhShsSFFNShLhnnvPtvSNNs
dzbmwVwbbBmGcqDbgllJCC
ndnvvzJDHvzHHHjnHjCCSDLgbSFwNFVbFVTL
mcQmQtpWTQGlmpTtMtqtpqTFsLcVSSscNCwLNLbwbbLNwV
pRZTQlhmtGWqqWnPHdnhjHrBjPPd
TtLpNHspTcLNNsLpthhsfmtjRSRlWWbzSwSRGwbWlWSSvlmS
qBJVnZZdJVZrZndbPbWwRzSMVGbbVS
QCdnBFBndBQDnrqrnqqNhpNNLpHthsThjGCTLL
phCgcdrFbPLpgrbFHqQqzzlbGWGqQbHW
SFTvTnVVMRnNTNfSHjHQMDlHwDWlQwDz
TZmTvsFNmvTtsggpdJLBBsCs
PBBWQjvsPsHVsNMcSzNDjcGggS
ZtrTfTrrrrdCqpdtLNnMLLqNcgMzgHLq
CZFmdTrJtbZrBvWHVVvHbPQW
djcrrBljMrTdCTcdCClClMlqRvtNqqSRwFbNbwvNBNpSzq
QhPmGJnPVGVHHNzSqpzFwztF
nhgPFmsnLPGLhPDJhGTcDjMfrMMjMZWfjfWj";
