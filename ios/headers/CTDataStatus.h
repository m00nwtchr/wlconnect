/*
* This header is generated by classdump-dyld 1.0
* on Wednesday, November 15, 2023 at 5:36:37â€¯PM Eastern European Standard Time
* Operating System: Version 17.1 (Build 21B74)
* Image Source: /System/Library/Frameworks/CoreTelephony.framework/CoreTelephony
* classdump-dyld is licensed under GPLv3, Copyright Â© 2013-2016 by Elias Limneos.
*/

@interface CTDataStatus : NSObject <NSCopying, NSSecureCoding> {

	BOOL _attached;
	BOOL _dataSim;
	BOOL _roamAllowed;
	BOOL _dataPlanSignalingReductionOverride;
	BOOL _cellularDataPossible;
	BOOL _inHomeCountry;
	int _indicator;
	int _indicatorOverride;
	int _radioTechnology;
	int _dataMode;
	int _dataBearerTechnology;
	unsigned _dataBearerSoMask;
	unsigned _activeContexts;
	unsigned _totalActiveContexts;
	int _reason;

}

@property (nonatomic,readonly) BOOL newRadioCoverage; 
@property (nonatomic,readonly) BOOL newRadioSaCoverage; 
@property (nonatomic,readonly) BOOL newRadioNsaCoverage; 
@property (nonatomic,readonly) BOOL newRadioDataBearer; 
@property (nonatomic,readonly) BOOL newRadioSaDataBearer; 
@property (nonatomic,readonly) BOOL newRadioNsaDataBearer; 
@property (nonatomic,readonly) BOOL newRadioMmWaveDataBearer; 
@property (nonatomic,readonly) BOOL newRadioSub6DataBearer; 
@property (assign,nonatomic) BOOL attached;                                        //@synthesize attached=_attached - In the implementation block
@property (assign,nonatomic) BOOL dataSim;                                         //@synthesize dataSim=_dataSim - In the implementation block
@property (assign,nonatomic) int indicator;                                        //@synthesize indicator=_indicator - In the implementation block
@property (assign,nonatomic) int indicatorOverride;                                //@synthesize indicatorOverride=_indicatorOverride - In the implementation block
@property (assign,nonatomic) BOOL roamAllowed;                                     //@synthesize roamAllowed=_roamAllowed - In the implementation block
@property (assign,nonatomic) int radioTechnology;                                  //@synthesize radioTechnology=_radioTechnology - In the implementation block
@property (assign,nonatomic) int dataMode;                                         //@synthesize dataMode=_dataMode - In the implementation block
@property (assign,nonatomic) int dataBearerTechnology;                             //@synthesize dataBearerTechnology=_dataBearerTechnology - In the implementation block
@property (assign,nonatomic) unsigned dataBearerSoMask;                            //@synthesize dataBearerSoMask=_dataBearerSoMask - In the implementation block
@property (assign,nonatomic) BOOL dataPlanSignalingReductionOverride;              //@synthesize dataPlanSignalingReductionOverride=_dataPlanSignalingReductionOverride - In the implementation block
@property (assign,nonatomic) BOOL cellularDataPossible;                            //@synthesize cellularDataPossible=_cellularDataPossible - In the implementation block
@property (assign,nonatomic) unsigned activeContexts;                              //@synthesize activeContexts=_activeContexts - In the implementation block
@property (assign,nonatomic) unsigned totalActiveContexts;                         //@synthesize totalActiveContexts=_totalActiveContexts - In the implementation block
@property (assign,nonatomic) BOOL inHomeCountry;                                   //@synthesize inHomeCountry=_inHomeCountry - In the implementation block
@property (assign,nonatomic) int reason;                                           //@synthesize reason=_reason - In the implementation block
+(BOOL)supportsSecureCoding;
-(BOOL)newRadioDataBearer;
-(void)setActiveContexts:(unsigned)arg1 ;
-(BOOL)attached;
-(BOOL)roamAllowed;
-(void)setDataSim:(BOOL)arg1 ;
-(void)setReason:(int)arg1 ;
-(int)reason;
-(int)dataMode;
-(void)setIndicator:(int)arg1 ;
-(BOOL)inHomeCountry;
-(unsigned)activeContexts;
-(void)encodeWithCoder:(id)arg1 ;
-(BOOL)cellularDataPossible;
-(int)dataBearerTechnology;
-(BOOL)newRadioSub6DataBearer;
-(BOOL)newRadioCoverage;
-(BOOL)dataPlanSignalingReductionOverride;
-(id)initWithCoder:(id)arg1 ;
-(void)setDataPlanSignalingReductionOverride:(BOOL)arg1 ;
-(BOOL)newRadioNsaCoverage;
-(void)setTotalActiveContexts:(unsigned)arg1 ;
-(id)copyBasic;
-(void)setIndicatorOverride:(int)arg1 ;
-(BOOL)newRadioNsaDataBearer;
-(void)setInHomeCountry:(BOOL)arg1 ;
-(void)setRadioTechnology:(int)arg1 ;
-(void)setCellularDataPossible:(BOOL)arg1 ;
-(BOOL)newRadioMmWaveDataBearer;
-(void)setDataBearerTechnology:(int)arg1 ;
-(int)indicator;
-(unsigned)dataBearerSoMask;
-(void)setAttached:(BOOL)arg1 ;
-(id)description;
-(void)setRoamAllowed:(BOOL)arg1 ;
-(id)copyWithZone:(NSZone*)arg1 ;
-(BOOL)dataSim;
-(void)setDataMode:(int)arg1 ;
-(unsigned)totalActiveContexts;
-(int)radioTechnology;
-(BOOL)newRadioSaDataBearer;
-(int)indicatorOverride;
-(void)setDataBearerSoMask:(unsigned)arg1 ;
-(BOOL)newRadioSaCoverage;
@end

