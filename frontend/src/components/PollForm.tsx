import React, { useCallback } from 'react';
import TextField from '@material-ui/core/TextField';
import InputAdornment from '@material-ui/core/InputAdornment';
import IconButton from '@material-ui/core/IconButton';
import Typography from '@material-ui/core/Typography';
import DeleteIcon from '@material-ui/icons/Delete';
import Button from '@material-ui/core/Button';
import ExpansionPanel from '@material-ui/core/ExpansionPanel';
import ExpansionPanelSummary from '@material-ui/core/ExpansionPanelSummary';
import ExpansionPanelDetails from '@material-ui/core/ExpansionPanelDetails';
import FormControlLabel from '@material-ui/core/FormControlLabel';
import Switch from '@material-ui/core/Switch';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import ChipInput from 'material-ui-chip-input';
import OutlinedInput from '@material-ui/core/OutlinedInput';
import MenuItem from '@material-ui/core/MenuItem';
import FormControl from '@material-ui/core/FormControl';
import Select from '@material-ui/core/Select';
import Box from '@material-ui/core/Box';
import DateFnsUtils from '@date-io/date-fns';
import { Slider } from 'material-ui-slider';
import { DateTimePicker, MuiPickersUtilsProvider } from '@material-ui/pickers';
import { useMappedState, useDispatch, pollForm, chains } from '../store';

export default function PollForm() {
    const state = useMappedState(useCallback(pollForm.getState, []));
    const allChains = useMappedState(useCallback(chains.getAllOk, []));
    const dispatch = useDispatch();
    const lastIndex = state.options.length - 1;
    return (
        <MuiPickersUtilsProvider utils={DateFnsUtils}>
            <form
                onSubmit={(e) => {
                    e.preventDefault();
                    dispatch(pollForm.submit());
                }}
            >
                <ExpansionPanel expanded>
                    <ExpansionPanelDetails>
                        <div>
                            <TextField
                                placeholder='What would you like to ask?'
                                margin='normal'
                                variant='outlined'
                                autoFocus
                                fullWidth
                                value={state.name}
                                onChange={(e) =>
                                    dispatch(pollForm.setName(e.target.value))
                                }
                            />

                            {state.options.map((option, index) => (
                                <TextField
                                    variant='outlined'
                                    fullWidth
                                    margin='dense'
                                    placeholder={`Option ${index + 1}`}
                                    value={option}
                                    onChange={(e) => {
                                        dispatch(
                                            pollForm.setOption(
                                                index,
                                                e.target.value,
                                            ),
                                        );
                                    }}
                                    onFocus={(_e) => {
                                        if (index === lastIndex) {
                                            dispatch(pollForm.addOption());
                                        }
                                    }}
                                    InputProps={{
                                        endAdornment: (
                                            <InputAdornment position='end'>
                                                <IconButton
                                                    aria-label='Delete'
                                                    edge='end'
                                                    onClick={(_e) =>
                                                        dispatch(
                                                            pollForm.delOption(
                                                                index,
                                                            ),
                                                        )
                                                    }
                                                    disabled={lastIndex <= 1}
                                                >
                                                    <DeleteIcon fontSize='small' />
                                                </IconButton>
                                            </InputAdornment>
                                        ),
                                    }}
                                />
                            ))}
                            <FormControl fullWidth margin='normal'>
                                <Select
                                    value={state.chainId}
                                    onChange={(e) => {
                                        dispatch(
                                            pollForm.setChainId(e.target
                                                .value as string),
                                        );
                                    }}
                                    displayEmpty
                                    name='chain'
                                    input={<OutlinedInput labelWidth={0} />}
                                >
                                    <MenuItem value='' disabled>
                                        Select a Chain
                                    </MenuItem>
                                    {allChains.map((chain) => (
                                        <MenuItem value={chain.chainId}>
                                            {chain.displayName}
                                        </MenuItem>
                                    ))}
                                </Select>
                            </FormControl>
                        </div>
                    </ExpansionPanelDetails>
                </ExpansionPanel>

                {/* <ExpansionPanel>
                    <ExpansionPanelSummary
                        expandIcon={<ExpandMoreIcon />}
                        aria-controls='panel2a-content'
                        id='panel2a-header'
                    >
                        <Typography>Answer Requirements</Typography>
                    </ExpansionPanelSummary>
                    <ExpansionPanelDetails>
                        <Box width={1}>
                            <Slider
                                color='#bf4040'
                                min={1}
                                max={2}
                                defaultValue={[1, 1]}
                                range
                                scaleLength={1}
                            />
                        </Box>
                    </ExpansionPanelDetails>
                </ExpansionPanel>

                <ExpansionPanel>
                    <ExpansionPanelSummary
                        expandIcon={<ExpandMoreIcon />}
                        aria-controls='panel2a-content'
                        id='panel2a-header'
                    >
                        <Typography>Voter Requirements</Typography>
                    </ExpansionPanelSummary>
                    <ExpansionPanelDetails>
                        <Box width={1}>
                            <FormControlLabel
                                control={
                                    <Switch
                                        value='checkedB'
                                        color='primary'
                                        checked={state.useAllowList}
                                        onChange={(e) =>
                                            dispatch(
                                                pollForm.setUseAllowList(
                                                    e.target.checked,
                                                ),
                                            )
                                        }
                                    />
                                }
                                label='Allow list'
                            />
                            <ChipInput
                                classes={{}}
                                value={state.voterList}
                                onAdd={(chip) => {
                                    console.dir(11111, chip);
                                    dispatch(pollForm.addVoter(chip as string));
                                }}
                                onDelete={(_chip, index) => {
                                    dispatch(pollForm.delVoter(index));
                                }}
                                fullWidth
                                helperText='Only these accounts will be allowed to vote'
                            />
                        </Box>
                    </ExpansionPanelDetails>
                </ExpansionPanel>

                <ExpansionPanel>
                    <ExpansionPanelSummary
                        expandIcon={<ExpandMoreIcon />}
                        aria-controls='panel2a-content'
                        id='panel2a-header'
                    >
                        <Typography>Dates</Typography>
                    </ExpansionPanelSummary>
                    <ExpansionPanelDetails>
                        <div>
                            <DateTimePicker
                                clearable
                                autoOk
                                disablePast
                                label='Open Time'
                                onChange={(value) => {
                                    dispatch(
                                        pollForm.setOpenTime(value as Date),
                                    );
                                }}
                                value={state.openTime}
                                maxDate={state.closeTime}
                            />
                            <DateTimePicker
                                clearable
                                autoOk
                                disablePast
                                label='Close Time'
                                onChange={(value) => {
                                    dispatch(
                                        pollForm.setCloseTime(value as Date),
                                    );
                                }}
                                value={state.closeTime}
                                minDate={state.openTime}
                            />
                        </div>
                    </ExpansionPanelDetails>
                </ExpansionPanel> */}

                <Box
                    display='flex'
                    alignItems='center'
                    justifyContent='center'
                    m={2}
                >
                    <Button
                        variant='contained'
                        color='primary'
                        size='large'
                        type='submit'
                    >
                        Create Poll
                    </Button>
                </Box>
            </form>
        </MuiPickersUtilsProvider>
    );
}

function Submitting() {
    return <>Submitting</>;
}

function SubmitOk() {
    return <>Created poll</>;
}
