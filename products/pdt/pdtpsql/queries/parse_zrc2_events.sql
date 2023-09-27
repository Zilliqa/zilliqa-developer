create or replace view txn_with_events as
select *,
    jsonb_path_query(receipt, '$.event_logs[*]._eventname')#>>'{}' as event,
    jsonb_path_query(
        receipt,
        '$.event_logs[*].params'
    ) as params
from transactions;
create or replace view mints as
select id,
    block,
    event,
    parse_event_param(params, 'minter') as minter,
    parse_event_param(params, 'recipient') as recipient,
    parse_event_param(params, 'amount')::numeric(76, 38) as amount
from txn_with_events
where event = 'Minted';
create or replace view burns as
select id,
    block,
    event,
    parse_event_param(params, 'burner') as burner,
    parse_event_param(params, 'burn_account') as burn_account,
    parse_event_param(params, 'amount')::numeric as amount
from txn_with_events
where event = 'Burnt';
create or replace view authorize_operators as
select id,
    block,
    event,
    parse_event_param(params, 'authorizer') as authorizer,
    parse_event_param(params, 'authorized_operator') as authorized_operator
from txn_with_events
where event = 'AuthorizeOperatorSuccess';
create or replace view revoke_operators as
select id,
    block,
    event,
    parse_event_param(params, 'revoker') as revoker,
    parse_event_param(params, 'revoked_operator') as revoked_operator
from txn_with_events
where event = 'RevokeOperatorSuccess';
create or replace view increase_allowances as
select id,
    block,
    event,
    parse_event_param(params, 'token_owner') as token_owner,
    parse_event_param(params, 'spender') as spender,
    parse_event_param(params, 'new_alowance')::numeric as new_alowance
from txn_with_events
where event = 'IncreasedAllowance';
create or replace view decrease_allowances as
select id,
    block,
    event,
    parse_event_param(params, 'token_owner') as token_owner,
    parse_event_param(params, 'spender') as spender,
    parse_event_param(params, 'new_alowance')::numeric as new_alowance
from txn_with_events
where event = 'DecreasedAllowance';
create or replace view transfers as
select id,
    block,
    event,
    parse_event_param(params, 'sender') as sender,
    parse_event_param(params, 'recipient') as recipient,
    parse_event_param(params, 'amount')::numeric as amount
from txn_with_events
where event = 'TransferSuccess';
create or replace view transfer_froms as
select id,
    block,
    event,
    parse_event_param(params, 'initiator') as initiator,
    parse_event_param(params, 'sender') as sender,
    parse_event_param(params, 'recipient') as recipient,
    parse_event_param(params, 'amount')::numeric as amount
from txn_with_events
where event = 'TransferFromSuccess';
create or replace view operator_sends as
select id,
    block,
    event,
    parse_event_param(params, 'initiator') as initiator,
    parse_event_param(params, 'token_owner') as token_owner,
    parse_event_param(params, 'recipient') as recipient,
    parse_event_param(params, 'amount')::numeric as amount
from txn_with_events
where event = 'OperatorSendSuccess';