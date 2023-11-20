import React from 'react';

function DisclaimerModal(props: any) {
    return (
        <div id="disclaimer-modal" className="modal fade" tabIndex={-1} role="dialog" aria-labelledby="disclaimerModalLabel" aria-hidden="true">
            <div className="modal-dialog modal-dialog-scrollable modal-dialog-centered modal-lg" role="document">
                <div className="modal-content">
                    <div className="modal-header">
                        <h5 className="modal-title" id="disclaimerModalLabel">Disclaimer</h5>
                        <button type="button" className="close btn shadow-none" data-dismiss="modal" aria-label="Close">
                            <span aria-hidden="true">&times;</span>
                        </button>
                    </div>
                    <div className="modal-body">
                        <p>By participating in the staking of ZILs (“Staking Program”), each participating individual and organization ("Participant") accepts and agrees that, to the extent permitted by law, [Zilliqa] disclaims all liability, damages, cost, loss or expense (including, without limitation, legal fees, costs and expenses) to it in respect of its involvement in the Staking Program. Each Participant should carefully consider all factors involved in participating in the Staking Program, including, but not limited to, those listed below and, to the extent necessary, consult an appropriate professional or other expert (including an expert in cryptographic tokens or blockchain-based software systems). If any of the following considerations are unacceptable to a Participant, that Participant should not be involved in the Staking Program. These considerations are not intended to be exhaustive and should be used as guidance only.</p>
                        <ul>
                            <li><p>The Staking Program is an open source protocol made available to the public, and Zilliqa expressly disclaims any liability in respect of any actions, programs, applications, developments, and operations of the Staking Program.</p></li>
                            <li><p>Hackers, individuals, other malicious groups or organisations may attempt to interfere with the Zilliqa Blockchain System, the ZILs and the Staking Program in a variety of ways such as cryptographic attacks, malware attacks, denial of service attacks, consensus-based attacks, Sybil attacks, smurfing and spoofing.</p></li>
                            <li><p>The regulatory status of cryptographic tokens, blockchain and distributed ledger technology as well as its applications are unclear or unsettled in many jurisdictions and it is difficult to predict how or whether governments or regulatory agencies may implement changes to law or apply existing regulation with respect to such technology and its applications, including the Zilliqa Blockchain System, the ZILs and the Staking Program.</p></li>
                            <li><p>The ZILs are not intended to represent any formal or legally binding investment. Cryptographic tokens that possess value in public markets, such as Ether and Bitcoin, have demonstrated extreme fluctuations in price over short periods of time on a regular basis. Participants should be prepared to expect similar fluctuations in the price of the ZILs and Participants may experience a complete and permanent loss of their initial purchase.</p></li>
                        </ul>
                        <p>The ZILs are not intended to be securities (or any other regulated instrument) under the laws of any jurisdiction where they are intended to be, or will be, purchased or sold and no action has been or will be taken in any jurisdiction by Zilliqa Research or any of its affiliates that would permit a public offering, or any other offering under circumstances not permitted by applicable law of the ZILs, in any country or jurisdiction where action for that purpose is required. Accordingly, the ZILs may not be offered or sold, directly or indirectly, by any holder, in or from any country or jurisdiction, except in circumstances which will result in compliance with all applicable rules and regulations of any such country or jurisdiction.</p>
                    </div>
                    <div className="modal-footer">
                        <button type="button" className="btn btn-user-action mx-auto d-block shadow-none" data-dismiss="modal">Okay</button>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default DisclaimerModal;