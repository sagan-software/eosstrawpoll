import blue from '@material-ui/core/colors/blue';
import { createMuiTheme, Theme } from '@material-ui/core/styles';
import {
    default as makeMuiStyles,
    StylesHook,
} from '@material-ui/styles/makeStyles';
import {
    Styles,
    ThemeOfStyles,
    WithStylesOptions,
} from '@material-ui/styles/withStyles';

export const theme = createMuiTheme({
    palette: {
        primary: { main: blue[900] },
    },
});

export function makeStyles<
    Props extends {} = {},
    ClassKey extends string = string
>(
    styles: Styles<Theme, Props, ClassKey>,
    options?: WithStylesOptions<Theme>,
): StylesHook<Styles<Theme, Props, ClassKey>> {
    return makeMuiStyles(styles, options);
}
