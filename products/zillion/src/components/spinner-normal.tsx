import React from 'react';


const SpinnerNormal = (props:any) => {
    return (
        <div className={props.class ? props.class: 'spinner-border'} role="status">
            <span className="sr-only">Loading...</span>
        </div>
    );
};

export default SpinnerNormal;