import React, { useState } from "react";

export interface LabelInfo {
  name: string;
  type: string;
  networkUrl: string;
  networkName: string;
  timeAdded: number;
}

export type LabelMap = Record<string, LabelInfo>;

export type NetworkMap = Map<string, string>;

type UserPrefState = {
  networkMap: NetworkMap;
  setNetworkMap: (newNetworkMap: NetworkMap) => void;
  labelMap: LabelMap;
  setLabelMap: (newLabelMap: LabelMap) => void;
};

export const UserPrefContext = React.createContext<UserPrefState | undefined>(
  undefined
);

export const UserPrefProvider = (props: { children: any }) => {
  const item = localStorage.getItem("networkMap");
  const [state, setState] = useState<UserPrefState>({
    networkMap: item ? new Map(JSON.parse(item)) : new Map(),
    setNetworkMap: (newNetworkMap) => {
      localStorage.setItem(
        "networkMap",
        JSON.stringify(Array.from(newNetworkMap.entries()))
      );
      setState((prevState) => ({ ...prevState, networkMap: newNetworkMap }));
    },
    labelMap: item ? JSON.parse(item) : {},
    setLabelMap: (newLabelMap) => {
      localStorage.setItem("labelMap", JSON.stringify(newLabelMap));
      setState((prevState) => ({ ...prevState, labelMap: newLabelMap }));
    },
  });

  return (
    <UserPrefContext.Provider value={state}>
      {props.children}
    </UserPrefContext.Provider>
  );
};
