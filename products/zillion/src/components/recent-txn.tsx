import React from 'react';
import { getTxnLink, getTransactionText } from '../util/utils';

function RecentTxnDropdown(props: any) {
    const data = props.data;
    const network = props.networkURL;

    return (
        <div id="txn-notify-dropdown">
            <div className="notification">
                <div className="notification-heading">
                    <h2>Recent Transactions</h2>
                </div>
                <div className="notification-wrapper">

                    { data.length === 0 &&
                        <p><em>No recent transactions found.</em></p> }

                    { data.map((item: { type: number, txnId: string }, index: number) =>
                        <div key={item.txnId ? item.txnId : index}>

                            { 
                                index === 0 &&
                                <h3 className="notification-subheading">Latest</h3>
                            }

                            {
                                index === 1 &&
                                <h3 className="notification-subheading">Others</h3>
                            }

                            <a href={getTxnLink(item.txnId, network)} className="notification-item-link" target="_blank" rel="noopener noreferrer">
                                <div className="notification-item">
                                    <h3 className="item-title">{getTransactionText(item.type)}</h3>
                                    <p className="item-info"><strong>Transaction ID</strong><br/>
                                        <span className="txn-id">{item.txnId ? item.txnId : "N/A"}</span>
                                    </p>
                                </div>
                            </a>

                        </div>
                    )}

                </div>
            </div>

        </div>
    );
}

export default RecentTxnDropdown;