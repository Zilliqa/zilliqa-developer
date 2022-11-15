import React from "react";

import { QueryPreservingLink } from "src/services/network/networkProvider";
import { OverlayTrigger, Tooltip } from "react-bootstrap";

import {
  hexAddrToZilAddr,
  zilAddrToHexAddr,
  stripHexPrefix,
} from "src/utils/Utils";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFileContract } from "@fortawesome/free-solid-svg-icons";

const ToAddrDispSimplified: any = ({ toAddr, txType, addr }: any) => {
  const hexAddr = stripHexPrefix(zilAddrToHexAddr(addr));

  let txTypeIcon: any = undefined;

  if (txType === "contract-creation") {
    txTypeIcon = (
      <FontAwesomeIcon color="darkturquoise" icon={faFileContract} />
    );
  }

  if (txType === "contract-call") {
    txTypeIcon = <FontAwesomeIcon color="darkorange" icon={faFileContract} />;
  }

  return (
    <OverlayTrigger
      placement="top"
      overlay={<Tooltip id={"overlay-to"}>{txType}</Tooltip>}
    >
      <div className="d-flex align-items-center">
        {txTypeIcon ? <div className="mr-2">{txTypeIcon}</div> : null}
        {txType === "contract-creation" ? (
          <div>Contract</div>
        ) : toAddr.toLowerCase() !== hexAddr ? (
          <QueryPreservingLink
            to={`/address/${hexAddrToZilAddr(toAddr)}`}
            className="ellipsis mono"
          >
            {hexAddrToZilAddr(toAddr)}
          </QueryPreservingLink>
        ) : (
          <span className="text-muted">{addr}</span>
        )}
      </div>
    </OverlayTrigger>
  );
};

export default ToAddrDispSimplified;
