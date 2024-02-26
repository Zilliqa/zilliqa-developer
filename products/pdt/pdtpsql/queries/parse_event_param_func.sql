CREATE FUNCTION parse_event_param(params jsonb, param_name text) RETURNS text AS $$
select jsonb_path_query(
        params,
        '$[*] ? (@.vname == $name).value',
        jsonb_build_object('name', param_name)
    )#>>'{}';
$$ LANGUAGE SQL;