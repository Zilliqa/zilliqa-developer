import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import useRecipientInput from "../hooks/useRecipientInput";
import { faRepeat } from "@fortawesome/free-solid-svg-icons";

export default function RecipientInput() {
  const { isAddressValid, toggleAddress, recipient, handleUpdateRecipient } =
    useRecipientInput();

  return (
    <div className="form-control">
      <div className="label">
        <span>Recipient</span>
      </div>
      <div className="join">
        <div className="indicator">
          <button
            className="btn join-item"
            disabled={!isAddressValid}
            onClick={() => toggleAddress()}
          >
            <FontAwesomeIcon
              icon={faRepeat}
              color="white"
              className="ml-auto"
            />
          </button>
        </div>
        <input
          className={`input join-item input-bordered w-full font-mono text-sm text-end ${
            !isAddressValid && "input-warning"
          }`}
          placeholder="Address"
          value={recipient}
          onChange={({ target }) => handleUpdateRecipient(target.value)}
        />
      </div>
      {!isAddressValid && (
        <div className="label align-bottom place-content-end">
          <span className="label-text-alt text-warning">Invalid Address</span>
        </div>
      )}
    </div>
  );
}
