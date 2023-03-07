import React, { useContext } from "react";
import { Container, Spinner } from "react-bootstrap";
import { Route, Switch } from "react-router-dom";

import HomePage from "src/components/HomePage/HomePage";
import DSBlocksPage from "src/components/ViewAllPages/DSBlocksPage";
import TxBlocksPage from "src/components/ViewAllPages/TxBlocksPage";
import TxnsPage from "src/components/ViewAllPages/TxnsPage";
import AddressDetailsPage from "src/components/DetailsPages/AddressDetailsPage/AddressDetailsPage";
import DSBlockDetailsPage from "src/components/DetailsPages/DSBlockDetailsPage/DSBlockDetailsPage";
import TxBlockDetailsPage from "src/components/DetailsPages/TxBlockDetailsPage/TxBlockDetailsPage";
import TxnDetailsPage from "src/components/DetailsPages/TxnDetailsPage/TxnDetailsPage";
import NetworkErrPage from "src/components/ErrorPages/NetworkErrPage";
import LabelsPage from "src/components/LabelsPage/LabelsPage";
import NetworksPage from "src/components/NetworksPage/NetworksPage";
import NotFoundPage from "src/components/ErrorPages/NotFoundPage";
import { NetworkContext } from "src/services/network/networkProvider";
import { Card, Button } from "react-bootstrap";
import { useLocation, Link } from "react-router-dom";
import {
  useNetworkUrl,
  useSearchParams,
} from "src/services/network/networkProvider";

import "./App.css";
const UnexplicableError: React.FC = () => {
  const location = useLocation();

  return (
    <>
      <Card className="error-card">
        <Card.Body>
          <h4 className="mb-3">Ups! Something went wrong.</h4>
          <h6 className="mb-2">
            Network: <strong>{useNetworkUrl()}</strong>
          </h6>
          <h6>
            Search: <strong>{useSearchParams()}</strong>
          </h6>
          <Link to={{ pathname: "/", search: location.search }}>
            <Button id="error-btn" className="mt-4">
              <span>Return to Dashboard</span>
            </Button>
          </Link>
        </Card.Body>
      </Card>
    </>
  );
};

class ErrorBoundary extends React.Component<
  { children: any },
  { hasError: boolean }
> {
  constructor(props: { children: any }) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: any) {
    // Update state so the next render will show the fallback UI.
    return { hasError: true };
  }

  componentDidCatch(error: any, errorInfo: any) {
    // You can also log the error to an error reporting service
    console.error(error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      // You can render any custom fallback UI
      return <UnexplicableError />;
    }

    return this.props.children;
  }
}

const App: React.FC = () => {
  const networkContext = useContext(NetworkContext);
  if (!networkContext) {
    return (
      <div className="center-spinner">
        <Spinner animation="border" />
      </div>
    );
  }

  const { inTransition, isValidUrl } = networkContext;

  return (
    <div className="app-container">
      <Container>
        <ErrorBoundary>
          {inTransition || isValidUrl === null ? (
            <div className="center-spinner">
              <Spinner animation="border" />
            </div>
          ) : (
            <>
              <Switch>
                <Route exact path="/labels">
                  <LabelsPage />
                </Route>
                <Route exact path="/networks">
                  <NetworksPage />
                </Route>
                {isValidUrl ? (
                  <>
                    <Switch>
                      <Route exact path="/">
                        <HomePage />
                      </Route>
                      <Route exact path="/dsbk">
                        <DSBlocksPage />
                      </Route>
                      <Route exact path="/txbk">
                        <TxBlocksPage />
                      </Route>
                      <Route exact path="/tx">
                        <TxnsPage />
                      </Route>
                      <Route path="/dsbk/:blockNum">
                        <DSBlockDetailsPage />
                      </Route>
                      <Route path="/txbk/:blockNum">
                        <TxBlockDetailsPage />
                      </Route>
                      <Route path="/tx/:txnHash">
                        <TxnDetailsPage />
                      </Route>
                      <Route path="/address/:addr">
                        <AddressDetailsPage />
                      </Route>
                      <Route>
                        <NotFoundPage />
                      </Route>
                    </Switch>
                  </>
                ) : (
                  <NetworkErrPage />
                )}
              </Switch>
            </>
          )}
        </ErrorBoundary>
      </Container>
    </div>
  );
};

export default App;
