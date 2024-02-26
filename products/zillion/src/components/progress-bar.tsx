import React from 'react';

function ProgressBar(props: any) {
    const { completed } = props;

    return (
        <div className="progress-bar-container">
            <div style={{width: `${completed}%`}} className="progress-bar-filler">
                <span className="progress-bar-label"></span>
            </div>
        </div>
    );
}

export default ProgressBar;
