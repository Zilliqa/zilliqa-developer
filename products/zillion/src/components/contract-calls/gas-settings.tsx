import BigNumber from 'bignumber.js';
import React from 'react';
import IconArrowDropdownLine from '../icons/arrow-dropdown-line';
import IconArrowDropupLine from '../icons/arrow-dropup-line';

function GasSettings(props: any) {
    const {
        gasOption, 
        gasPrice, 
        onBlurGasPrice, 
        onGasPriceChange,
        gasLimit,
        onBlurGasLimit,
        onGasLimitChange,
        setGasOption,
    } = props;

    const computeGasFees = () => {
        if (!gasPrice || !gasLimit) {
            return new BigNumber(0);
        }
        const fees = new BigNumber(gasPrice).multipliedBy(new BigNumber(gasLimit));
        return fees;
    }

    return (
        <div>
            {
                gasOption &&
                <div className="mt-4">
                    <h3 id="fee-section-heading">Adjust Gas Fees</h3>
                    <div className="row my-4">
                        <div className="col mr-4">
                            <div className="modal-label mb-2"><strong>Gas Price</strong></div>
                            <div className="input-group mb-2">
                                <input 
                                    type="text" 
                                    className="form-control shadow-none" 
                                    value={gasPrice} 
                                    onBlur={onBlurGasPrice}
                                    onChange={onGasPriceChange} 
                                />
                            </div>
                        </div>
                        <div className="col">
                            <div className="modal-label mb-2"><strong>Gas Limit</strong></div>
                            <div className="input-group mb-2">
                                <input 
                                    type="text" 
                                    className="form-control shadow-none" 
                                    value={gasLimit} 
                                    onBlur={onBlurGasLimit}
                                    onChange={onGasLimitChange}
                                />
                            </div>
                        </div>
                    </div>
                </div>
            }
            <div className="d-flex mb-4">
                <div>
                    <span className="fee-head"><strong>Fee</strong>:  <span className="fee">{ computeGasFees().shiftedBy(-12).toFixed() }</span> ZIL</span>
                </div>
                <button 
                    className="ml-auto btn advanced-btn shadow-none"
                    onClick={() => setGasOption(!gasOption)}>
                        {
                            gasOption ? 
                            <div className="advanced-btn-div"><IconArrowDropupLine width="28" height="28" className="pb-1" /><span>Advanced Settings</span></div> : 
                            <div className="advanced-btn-div"><IconArrowDropdownLine width="28" height="28" className="pb-1" /><span>Advanced Settings</span></div>
                        }
                </button>
            </div>
        </div>
    );
}

export default GasSettings