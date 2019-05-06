import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import Paper from '@material-ui/core/Paper';
import CheckBox from '@material-ui/icons/CheckBox';
import CheckBoxOutlineBlank from '@material-ui/icons/CheckBoxOutlineBlank';
import RadioButtonChecked from '@material-ui/icons/RadioButtonChecked';
import RadioButtonUnchecked from '@material-ui/icons/RadioButtonUnchecked';
import React from 'react';
import * as Store from '../store';

export interface Props {
    readonly selected: ReadonlyArray<string> | string;
    readonly onClick?: (chain: Store.Chains.Chain) => void;
    readonly chains: ReadonlyArray<Store.Chains.Chain>;
}

export default function ChainsList(props: Props) {
    const multiple = Array.isArray(props.selected);
    const isSelected = multiple
        ? (chainId: string) => props.selected.indexOf(chainId) >= 0
        : (chainId: string) => chainId === props.selected;
    return (
        <Paper>
            <List>
                {/* TODO: No chains */}
                {props.chains.map((chain) => (
                    <ListItem
                        button
                        key={chain.chainId}
                        onClick={() =>
                            props.onClick ? props.onClick(chain) : null
                        }
                    >
                        <ListItemIcon>
                            {isSelected(chain.chainId) ? (
                                multiple ? (
                                    <CheckBox />
                                ) : (
                                    <RadioButtonChecked />
                                )
                            ) : multiple ? (
                                <CheckBoxOutlineBlank />
                            ) : (
                                <RadioButtonUnchecked />
                            )}
                        </ListItemIcon>
                        <ListItemText primary={chain.displayName} />
                    </ListItem>
                ))}
            </List>
        </Paper>
    );
}
