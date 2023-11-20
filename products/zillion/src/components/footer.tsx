import React from 'react';


function Footer(props:any) {
    return (
        <footer id="disclaimer" className="align-items-start">
            <div className="p-2 d-block">
                <span className="mx-3 align-middle">&copy; 2023 Zilliqa</span> 
                <button type="button" className="btn shadow-none" data-toggle="modal" data-target="#disclaimer-modal" data-keyboard="false" data-backdrop="static">Disclaimer</button>
            </div>
      </footer>
    );
}

export default Footer;

