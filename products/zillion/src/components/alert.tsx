import React from 'react'
import { toast } from 'react-toastify';

import 'react-toastify/dist/ReactToastify.css';

// header: short heading desc of the message, e.g. wallet locked
// advice: action on what to do, e.g. please unlock your wallet
function ToastMsg({header, advice}: any) {
    return (
        <div id="toast-msg-container">
            <span className="toast-msg-header"><strong>{header}</strong></span>
            <br/>
            <span className="toast-msg-advice">{advice}</span>
        </div>
    );
}

// type is not used currently
// prepare for future use cases
function Alert(type: string, header: string, advice: string) {
    switch (type) {
        case 'info':
            return toast.info(<ToastMsg header={header} advice={advice} />);
        case 'error':
            return toast.error(<ToastMsg header={header} advice={advice} />);
        case 'success':
            return toast.success(<ToastMsg header={header} advice={advice} />);
        case 'warn':
            return toast.warn(<ToastMsg header={header} advice={advice} />);
        default:
            return toast(<ToastMsg header={header} advice={advice} />);
    }
    
}

export default Alert;