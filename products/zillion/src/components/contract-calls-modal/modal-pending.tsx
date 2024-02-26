import React from 'react';

const ModalPending = () => {
    return (
        <div className="modal-body modal-processing text-center">
            <h2>Processing...</h2>
            <p className="mt-4">Please wait 1 - 2 minutes while we process the request.</p>
            <div className='spinner-border dashboard-spinner' role="status">
                <span className="sr-only">Loading...</span>
            </div>
        </div>
    )
};

export default ModalPending;