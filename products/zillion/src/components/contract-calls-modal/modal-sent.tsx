import React from 'react';
import { getTxnLink } from '../../util/utils';

import IconCheckboxCircle from '../icons/checkbox-circle';

const ModalSent = (props: any) => {
    return (
        <div className="modal-body modal-sent text-center">
            <IconCheckboxCircle className="modal-icon-success" width="80" height="80" />
            <h2 className="mt-2">Transaction Sent</h2>
            <div className="txn-id">
                <a href={getTxnLink(props.txnId, props.networkURL)} target="_blank" rel="noopener noreferrer">{props.txnId}</a>
            </div>
            <button type="button" className="btn btn-user-action mt-2 mx-2" data-dismiss="modal" onClick={props.handleClose}>Done</button>
        </div>
    );
};

export default ModalSent;