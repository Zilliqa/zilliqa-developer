import React from 'react';
import DisclaimerModal from './disclaimer';
import IconNotAuthenticated from './icons/not-authenticated';
import useDarkMode from '../util/use-dark-mode';

function Error404(props: any) {

    const directUser = () => {
        props.history.replace("/");
    };

    // need this to set the correct theme
    // eslint-disable-next-line
    const darkMode = useDarkMode(true);

    return (
        <div className="cover">
            <div className="container-fluid">
                <div className="row align-items-center">
                    <div className="cover-content col-12">
                        <div id="unauthenticated-section" className="text-center mx-auto">
                            <IconNotAuthenticated className="home-icon" width="400px" />
                            <h3>Error</h3>
                            <p>Sorry, something seems to be broken.<br/>Please re-connect to your wallet again.</p>
                            <button type="button" className="btn btn-user-action mx-2" onClick={directUser}>Log In</button>
                        </div>
                    </div>
                    <footer id="disclaimer" className="align-items-start">
                        <div className="p-2">
                        <span className="ml-4 mx-3">&copy; 2023 Zilliqa</span> 
                        <button type="button" className="btn" data-toggle="modal" data-target="#disclaimer-modal" data-keyboard="false" data-backdrop="static">Disclaimer</button>
                        </div>
                    </footer>
                    <DisclaimerModal />
                </div>
            </div>
        </div>

    );
}

export default Error404;