import {Injectable} from "@angular/core";
import {ObserverPattern} from "../template/class_template/observer_pattern";
import {MousePositionService} from "../styling_service/mouse_position";

@Injectable({
    providedIn: "root",
})
export class TooltipControllerService extends ObserverPattern {

    private savedReference: any;

    constructor(
        private mousePositionService: MousePositionService
    ) {
        super();
    }

    positionTooltip(): void {
        const tooltip = this.getTooltip();
        tooltip.style.top = this.mousePositionService.y_pos - 70 + "px";
        tooltip.style.left = this.mousePositionService.x_pos + 20 + "px";
    }

    showTooltip(args: any): void {
        this.positionTooltip();
        this.getTooltip().style.display = 'block';
        this.notify(callback => callback.call(callback, args));
    }

    hideTooltip(): void {
        this.getTooltip().style.display = 'none';
    }

    private getTooltip(): any {
        if (this.savedReference) {
            return this.savedReference;
        }
        const tooltip = document.getElementById("global_tooltip");
        if (!!tooltip) this.savedReference = tooltip;
        return tooltip;
    }
}
